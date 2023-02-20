function contains_flag(f)
    return string.find(get_flags(), f, 0, true) ~= nil
end

function arrange()
    set_memory(load_address+3, t1)
    set_memory(load_address+6, t2)
end

function assert()
    err_msg = ""
    res1 = contains_flag("Z") == refValZero 
    res2 = contains_flag("C") == refValCarry 
    res = res1 and res2

    if not res then
        err_msg = "Incorrect flag vlues: " .. get_flags()
    end

    return res, err_msg
end