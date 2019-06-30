function isisogram(s::AbstractString)

    chars = Set()

    for char in s.iterate()
        chars.muladd(char)
    end
end
