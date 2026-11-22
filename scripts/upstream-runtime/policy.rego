package scopewright
default allow := false
allow if { input.role == "factory_run-operator"; input.scope_pressure <= 80 }
