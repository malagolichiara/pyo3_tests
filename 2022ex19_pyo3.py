_input = """Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 15 clay. Each geode robot costs 3 ore and 9 obsidian.
Blueprint 2: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 12 clay. Each geode robot costs 4 ore and 19 obsidian.
Blueprint 3: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 14 clay. Each geode robot costs 3 ore and 16 obsidian.
Blueprint 4: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 17 clay. Each geode robot costs 3 ore and 19 obsidian.
Blueprint 5: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 10 clay. Each geode robot costs 2 ore and 10 obsidian.
Blueprint 6: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 16 clay. Each geode robot costs 3 ore and 9 obsidian.
Blueprint 7: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 7 clay. Each geode robot costs 3 ore and 9 obsidian.
Blueprint 8: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 19 clay. Each geode robot costs 2 ore and 12 obsidian.
Blueprint 9: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 16 clay. Each geode robot costs 3 ore and 20 obsidian.
Blueprint 10: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 7 clay. Each geode robot costs 2 ore and 14 obsidian.
Blueprint 11: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 4 ore and 12 obsidian.
Blueprint 12: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 15 clay. Each geode robot costs 2 ore and 15 obsidian.
Blueprint 13: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 5 clay. Each geode robot costs 2 ore and 10 obsidian.
Blueprint 14: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 15 clay. Each geode robot costs 3 ore and 16 obsidian.
Blueprint 15: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 19 clay. Each geode robot costs 2 ore and 9 obsidian.
Blueprint 16: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 15 clay. Each geode robot costs 4 ore and 9 obsidian.
Blueprint 17: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 17 clay. Each geode robot costs 3 ore and 16 obsidian.
Blueprint 18: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 13 clay. Each geode robot costs 3 ore and 12 obsidian.
Blueprint 19: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 10 clay. Each geode robot costs 4 ore and 10 obsidian.
Blueprint 20: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 17 clay. Each geode robot costs 3 ore and 10 obsidian.
Blueprint 21: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 22: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 3 ore and 14 obsidian.
Blueprint 23: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 17 clay. Each geode robot costs 2 ore and 10 obsidian.
Blueprint 24: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 19 clay. Each geode robot costs 3 ore and 19 obsidian.
Blueprint 25: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 14 clay. Each geode robot costs 3 ore and 14 obsidian.
Blueprint 26: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 13 clay. Each geode robot costs 3 ore and 19 obsidian.
Blueprint 27: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 7 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 28: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 20 clay. Each geode robot costs 2 ore and 19 obsidian.
Blueprint 29: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 16 clay. Each geode robot costs 2 ore and 15 obsidian.
Blueprint 30: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 19 clay. Each geode robot costs 2 ore and 18 obsidian."""

import re


def parser2(s):
    ret = []
    for line in s.splitlines():
        parsed_line = re.findall(
            r"Blueprint \d+: Each (\w+) robot costs (\d+) (\w+). Each (\w+) robot costs (\d+) (\w+). Each (\w+) robot costs (\d+) (\w+) and (\d+) (\w+). Each (\w+) robot costs (\d+) (\w+) and (\d+) (\w+).",
            line,
            re.I | re.S,
        )[0]
        # print(parsed_line)
        d = {}
        inside = {}
        inside["ore"] = int(parsed_line[1])
        d["ore"] = inside
        inside = {}
        inside["ore"] = int(parsed_line[4])
        d["clay"] = inside
        inside = {}
        inside["ore"] = int(parsed_line[7])
        inside["clay"] = int(parsed_line[9])
        d["obsidian"] = inside
        inside = {}
        inside["ore"] = int(parsed_line[12])
        inside["obsidian"] = int(parsed_line[14])
        d["geode"] = inside

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

print("pyo3")

import time

t0 = time.monotonic()
ret = 0
for i, blueprint in enumerate(parser2(_input)):
    n = pyo3_tests.simulation(blueprint, 24)
    print(i, n)
    ret += (i + 1) * n

print(time.monotonic() - t0, ret)

input()
