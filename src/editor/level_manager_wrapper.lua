return function(internal_functions)
    local lm = {}

    for name, func in pairs(internal_functions) do
        lm[name] = function(...)
            return coroutine.yield(func(...))
        end
    end

    return lm
end