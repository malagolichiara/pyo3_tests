_input = """Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 15 clay. Each geode robot costs 3 ore and 9 obsidian.
Blueprint 2: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 12 clay. Each geode robot costs 4 ore and 19 obsidian.
Blueprint 3: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 14 clay. Each geode robot costs 3 ore and 16 obsidian."""

import re

def parser2(s):
    ret = []
    for line in s.splitlines():
        parsed_line = re.findall(r'Blueprint \d+: Each (\w+) robot costs (\d+) (\w+). Each (\w+) robot costs (\d+) (\w+). Each (\w+) robot costs (\d+) (\w+) and (\d+) (\w+). Each (\w+) robot costs (\d+) (\w+) and (\d+) (\w+).',
                         line,
                         re.I|re.S)[0]
        # print(parsed_line)
        d = {}
        inside = {}
        inside['ore'] = int(parsed_line[1])
        d['ore'] = inside
        inside = {}
        inside['ore'] = int(parsed_line[4])
        d['clay'] = inside
        inside = {}
        inside['ore'] = int(parsed_line[7])
        inside['clay'] = int(parsed_line[9])
        d['obsidian'] = inside
        inside = {}
        inside['ore'] = int(parsed_line[12])
        inside['obsidian'] = int(parsed_line[14])
        d['geode'] = inside

        ret.append(d)
        # ret.append(
        #     {
        #         'ore': {'ore': int(parsed_line[1])},
        #         'clay': {'ore': int(parsed_line[4])},
        #         'obsidian': {'ore': int(parsed_line[7]),
        #                      'clay': int(parsed_line[9])},
        #         'geode': {'ore': int(parsed_line[12]),
        #                      'obsidian': int(parsed_line[14])},
        #     }
        # )
    return ret

import pyo3_tests

import time
t0 = time.monotonic()
ret = 1
for i, blueprint in enumerate(parser2(_input)):
    n = pyo3_tests.simulation(blueprint, 32)
    print(i,n)
    ret *= n

print(time.monotonic()-t0, ret)
