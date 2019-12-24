"""Generate the ops files for reckoner types (Integer, Rational)

The structure of the ops description JSON file is:
```
{
    "template": <template filename>,
    "output": <path to output relative to rust `src` folder>,
    "trait": {
        "name": <Add|Sub|Mul|...>,
        "fn_name": <add|sub|mul|...>,
        "is_op_assign": <true|false>,
    },
    "instances": [
        # one of the following
        # most customizable
        # array with len 4
        # lhs - 0, rhs - 1, return - 2, body - 3
        # conceptually equal to this
        {
            "lhs": <lhs type>, "rhs": <rhs type>, "return": <return type>,
            "body": <string containing the text of the impl body>
        },
        # or
        # array with len 7
        # lhs - 0, rhs - 1, return - 2, inner_fn - 3, mut_inplace_return - 4,
        # lhs_effects - 5, rhs_effects - 6
        # conceptually equal to this
        {
            "lhs": <lhs type>, "rhs": <rhs type>, "return": <return type>,
            "inner_fn": {
                "name": <string>,
                "mut_inplace_return": <no|lhs|rhs>,
            },
            "lhs_effects": [
                # Applied from inside out
                <deref|ref|ref_mut|{ "convert": <type name implementing From<$lhs>> }>,
                ...
            ],
            "rhs_effects": [
                <deref|ref|ref_mut|{ "convert": <type name implementing From<$rhs>> }>,
                ...
            ]
        },
        # or
        # dict with key 'condition'
        {
            'condition': <cfg condition like: all(target_pointer_width = "64", not(windows))>,
            'if_instances': [
                <same as outer array, but can't nest condition objects>,
                ...
            ],
            'else_instances': [
                <same as outer array, but can't nest condition objects>,
                ...
            ]
        }
    ]
}
```
"""

import sys
import json
import os
import subprocess
from copy import copy

INSTANCE_SEPARATOR = "\n"

INSTANCE_OUTER_TEMPLATE = r"""
// {options_comment}
impl {trait}<{rhs}> for {lhs} {{
    type Output = {return};

    fn {fn_name}({lhs_value}, {rhs_value}: {rhs}) -> Self::Output {{
{fn_body}
    }}
}}
"""

CONDITIONED_OUTER_TEMPLATE = r"""
// {options_comment}
#[cfg({condition})]
impl {trait}<{rhs}> for {lhs} {{
    type Output = {return};

    fn {fn_name}({lhs_value}, {rhs_value}: {rhs}) -> Self::Output {{
{fn_body}
    }}
}}
"""

ASSIGN_INSTANCE_OUTER_TEMPLATE = r"""
// {options_comment}
impl {trait}<{rhs}> for {lhs} {{
    fn {fn_name}(&mut {lhs_value}, {rhs_value}: {rhs}) {{
{fn_body}
    }}
}}
"""

ASSIGN_CONDITIONED_OUTER_TEMPLATE = r"""
// {options_comment}
#[cfg({condition})]
impl {trait}<{rhs}> for {lhs} {{
    fn {fn_name}(&mut {lhs_value}, {rhs_value}: {rhs}) {{
{fn_body}
    }}
}}
"""

# map from (is_op_assign, is_manual_impl) to FIELD_POS
FIELD_POS_MAP = {
    (True, True): [
        'lhs', 'rhs', 'body'
    ],
    (True, False): [
        'lhs', 'rhs', 'inner_fn', 'mut_inplace_return', 'lhs_effects', 'rhs_effects'
    ],
    (False, True): [
        'lhs', 'rhs', 'return', 'body'
    ],
    (False, False): [
        'lhs', 'rhs', 'return', 'inner_fn', 'mut_inplace_return', 'lhs_effects', 'rhs_effects'
    ],
}


class FieldPos:
    def __init__(self, is_op_assign, is_manual_impl):
        self._field_pos = FIELD_POS_MAP[(
            is_op_assign, is_manual_impl)]

    def __getitem__(self, key):
        return self._field_pos.index(key)


def apply_value_effect(effect, value):
    if effect == 'deref':
        return "*{}".format(value)
    elif effect == "ref":
        return "&{}".format(value)
    elif effect == "ref_mut":
        return "&mut {}".format(value)
    elif isinstance(effect, dict) and 'convert' in effect:
        return "{}::from({})".format(effect['convert'], value)
    else:
        raise RuntimeError("Unknown value effect: '{}'".format(effect))


def apply_all_effects(effects, initial_value):
    effects = copy(effects)
    effects.reverse()
    value = initial_value

    for effect in effects:
        value = apply_value_effect(effect, value)

    return value


REGULAR_BINOP_FN_BODY_TEMPLATE = "\
        {inner_fn}({lhs_value}, {rhs_value})"

MUT_INPLACE_RETURN_FN_BODY_TEMPLATE = "\
        {inner_fn}({lhs_value}, {rhs_value});\
\
        {ret_value}"


def is_manual_instance_impl(instance_desc):
    return len(instance_desc) == 4


def generate_impl_instance(field_pos, instance_desc):
    if is_manual_instance_impl(instance_desc):
        # body is ready to go
        return instance_desc[field_pos['body']]
    else:
        # got to generate the body
        inner_fn = instance_desc[field_pos['inner_fn']]
        mut_inplace_return = instance_desc[field_pos['mut_inplace_return']]

        lhs_value = apply_all_effects(
            instance_desc[field_pos['lhs_effects']], "self")
        rhs_value = apply_all_effects(
            instance_desc[field_pos['rhs_effects']], "rhs")

        format_kwargs = {
            'inner_fn': inner_fn,
            'lhs_value': lhs_value,
            'rhs_value': rhs_value
        }

        if mut_inplace_return == "no":
            body_template = REGULAR_BINOP_FN_BODY_TEMPLATE
        else:
            if mut_inplace_return == 'lhs':
                format_kwargs['ret_value'] = 'self'
            elif mut_inplace_return == 'rhs':
                format_kwargs['ret_value'] = 'rhs'

            body_template = MUT_INPLACE_RETURN_FN_BODY_TEMPLATE

        return body_template.format(**format_kwargs)


def apply_instance_desc(trait_desc, instance_desc, template, condition=None):
    trait = trait_desc['name']
    fn_name = trait_desc['fn_name']
    is_op_assign = bool(trait_desc['is_op_assign'])
    is_manual_impl = is_manual_instance_impl(instance_desc)
    field_pos = FieldPos(is_op_assign, is_manual_impl)

    format_kwargs = {
        'options_comment': str(instance_desc),
        'lhs': instance_desc[field_pos['lhs']],
        'lhs_value': 'self',
        'rhs': instance_desc[field_pos['rhs']],
        'rhs_value': 'rhs',
        'trait': trait,
        'fn_name': fn_name,
        'fn_body': generate_impl_instance(field_pos, instance_desc)
    }

    if not is_op_assign:
        format_kwargs['return'] = instance_desc[field_pos['return']]

    if not is_manual_instance_impl(instance_desc):
        # apply mut tag if the value will be mutably borrowed
        if instance_desc[field_pos['mut_inplace_return']] == 'lhs':
            format_kwargs['lhs_value'] = "mut {}".format(
                format_kwargs['lhs_value'])

        # apply mut tag if the value will be mutably borrowed
        if instance_desc[field_pos['mut_inplace_return']] == 'rhs':
            format_kwargs['rhs_value'] = "mut {}".format(
                format_kwargs['rhs_value'])

    if condition is not None:
        format_kwargs['condition'] = condition

    return template.format(**format_kwargs)


def generate_all_instances(trait_desc, instances):
    is_op_assign = bool(trait_desc['is_op_assign'])

    if is_op_assign:
        unconditioned_template = ASSIGN_INSTANCE_OUTER_TEMPLATE
        conditioned_template = ASSIGN_CONDITIONED_OUTER_TEMPLATE
    else:
        unconditioned_template = INSTANCE_OUTER_TEMPLATE
        conditioned_template = CONDITIONED_OUTER_TEMPLATE

    for instance_desc in instances:
        if isinstance(instance_desc, dict) and 'condition' in instance_desc:
            if_condition = instance_desc['condition']
            else_condition = "not({})".format(if_condition)
            if_instances = instance_desc['if_instances']
            else_instances = instance_desc['else_instances']

            for if_instance_desc in if_instances:
                yield apply_instance_desc(trait_desc, if_instance_desc,
                                          conditioned_template, condition=if_condition)

            for else_instance_desc in else_instances:
                yield apply_instance_desc(trait_desc, else_instance_desc,
                                          conditioned_template, condition=else_condition)
        else:
            yield apply_instance_desc(trait_desc, instance_desc, unconditioned_template)


def main(args):
    op_description_file = args[0]
    template_path = args[1]
    src_path = args[2]

    with open(op_description_file, 'r') as op_description_file:
        op_description = json.load(op_description_file)
        trait_desc = op_description['trait']
        trait_instances = op_description['instances']

        template_filename = os.path.join(
            template_path, op_description['template'])
        output_path = os.path.join(src_path, op_description['output'])

        with open(output_path, 'w') as output_file, open(
            template_filename, 'r'
        ) as template_file:
            template_string = template_file.read()

            all_instances_source = INSTANCE_SEPARATOR.join(
                generate_all_instances(trait_desc, trait_instances))

            generated_source = template_string.format(all_instances_source)

            output_file.write(generated_source)

    # Run rustfmt on file just produced
    subprocess.call(["rustfmt", output_path])


if __name__ == "__main__":
    if len(sys.argv) != 4:
        print("usage: python generate_integer_ops.py <ops_description.json> <template folder> <src path>")
    else:
        main(sys.argv[1:])
