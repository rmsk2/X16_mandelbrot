function contains_flag(f)
    return string.find(get_flags(), f, 0, true) ~= nil
end


function arrange()
    set_memory(load_address+3, t1)
    set_memory(load_address+6, t2)
    set_memory(load_address+9, tdiff)
end

function assert()
    res = contains_flag("Z")

    if not res then
        err_msg = "Time intervals not equal" 
    end

    return res, err_msg
end