class Test
    def inc(x)

        def abs(y)
            y.abs
        end

        abs(x) + 1
    end
end

if __FILE__ == $0
    a = Test.new
    negate = lambda { |x| -x }
    puts(a.inc(negate.call(1)))
end