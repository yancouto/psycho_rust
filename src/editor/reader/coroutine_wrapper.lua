-- This function wraps all event functions of the given table in coroutine yields
-- We use this since this is not doable from the rlua API

return function(internal_functions)
    local ret = {}

    for name, func in pairs(internal_functions) do
        assert(type(func) == "function")
        ret[name] = function(...)
            return coroutine.yield(func(...))
        end
    end

    return ret
end