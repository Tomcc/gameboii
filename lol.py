
import json

opcodes = json.loads(open("opcodes.json", 'r').read())

with open('newcodes.json', 'wt') as f:
    f.write("{")
    for i in range(0, len(opcodes)):
        elem = opcodes[i]

        opcode = elem["opcode"][2:]
        del elem["opcode"]

        prefix = ""
        if "prefix" in elem:
            prefix = elem["prefix"][2:]
            del elem["prefix"]

        f.write('"0x' + prefix + opcode + '":')

        json.dump(elem, f, ensure_ascii=False)

        if i != len(opcodes) - 1:
            f.write(",")

    f.write("}")
    