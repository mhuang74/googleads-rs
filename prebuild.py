#!/usr/bin/python3
# -*- coding: utf-8 -*-
from functools import reduce
import subprocess
import os
from jinja2 import Environment, FileSystemLoader


def getFromDict(dataDict, mapList):
    def reducer(acc, x):
        if (not x in acc):
            acc[x] = {}
        return acc[x]
    return reduce(reducer, mapList, dataDict)


def setInDict(dataDict, mapList, value):
    getFromDict(dataDict, mapList[:-1])[mapList[-1]] = value


def main():
    env = Environment(
        loader=FileSystemLoader('./templates/', encoding='utf8'),
        trim_blocks=True,
        lstrip_blocks=True)

    res = subprocess.run(
        './utils/services.sh', stdout=subprocess.PIPE)

    cmd_lines = res.stdout.decode('utf-8').split('\n')[:-1]

    #
    # lib.rs
    #

    packages_list = [x.split(' ')[0] for x in cmd_lines]

    mods = []
    for i in range(len(packages_list) - 1):
        p1 = packages_list[i]
        p2 = packages_list[i + 1]
        if p1 == p2:
            packages_list[i] = None
        if p2.startswith(p1 + '.'):
            packages_list[i] = None
            mods.append(p1)

    ignore_packages = ['google.firebase.fcm.connection.v1alpha1']
    for package in ignore_packages:
        packages_list.remove(package)

    packages = {}
    for package in packages_list:
        if package is None:
            continue
        setInDict(packages, package.split('.'), package)
    for package in mods:
        package_split = package.split('.')
        package_split.append('self')
        setInDict(packages, package_split, package)

    tpl_lib_rs = env.get_template('lib.rs.jinja')
    txt_lib_rs = tpl_lib_rs.render({'packages': packages})

    #
    # build.rs
    #

    paths = [x.split(' ')[1] for x in cmd_lines]
    tpl_build_rs = env.get_template('build.rs.jinja')
    txt_built_rs = tpl_build_rs.render({'paths': paths})

    #
    # write
    #

    os.makedirs('src', exist_ok=True)

    with open('src/lib.rs', 'w') as f:
        f.write(txt_lib_rs)

    with open('build.rs', 'w') as f:
        f.write(txt_built_rs)

    subprocess.run(['cargo', 'fmt'])
    print('Success.')


if __name__ == "__main__":
    main()
