#!/usr/bin/env julia

using Pkg
Pkg.activate(@__DIR__)
using Orchestrator

install = "--install" in ARGS
Orchestrator.main(install)
