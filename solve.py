#!/usr/bin/env python

import angr
import sys

if sys.argv[1] == "main":
  p = angr.Project("build/main")
elif sys.argv[1] == "bench":
  p = angr.Project("build/bench")

state = p.factory.full_init_state()
state.options.add(angr.options.BYPASS_UNSUPPORTED_SYSCALL)

sm = p.factory.simulation_manager(state)
sm.use_technique(angr.exploration_techniques.dfs.DFS())

dep = 0
while len(sm.active) > 0:
  dep += 1
  sm.step()
  print(dep, sm, len(sm.deadended + sm.active))
  for s in sm.deadended + sm.active:
    stdin = s.posix.dumps(0)
    stdout = s.posix.dumps(1)
    if stdin and ("yes" in stdout):
      print stdin
      exit()
