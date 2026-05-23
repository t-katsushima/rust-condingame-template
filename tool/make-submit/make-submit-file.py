#! /usr/bin/env python3

import os

from sqlalchemy import null

# #![allow(non_snake_case)] や mod common; の削除
def remove_template(lines):
    while lines:
        line = lines[0]
        if line.startswith("#![") or line.startswith("mod "):
            del lines[0]
        else:
            break

# サブミットファイルの用意
submit_file_name = '../../src/submit.rs'
if os.path.exists(submit_file_name):
    os.remove(submit_file_name)

with open(submit_file_name, 'x') as submit_file:
    submit_file.write("#![allow(non_snake_case)]\n")

    target_files = null
    with open("./target-file-list.txt") as target_file_list:
        for file_name in target_file_list.readlines():
            file_name = file_name.strip()
            with open("../../src/{}.rs".format(file_name), 'r') as source_file:
                lines = source_file.readlines()
                remove_template(lines)
                text = (
                    "mod {} {{".format(file_name),
                    "".join(lines),
                    "}"
                )

                text = "\n".join(text) + "\n"
                submit_file.write(text)

    with open('../../src/main.rs', 'r') as main_file:
        lines = main_file.readlines()
        remove_template(lines)
        submit_file.write("".join(lines))
