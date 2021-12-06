using Formatting

function a()
	f = open("input.txt")

	x, y = 0, 0

	for line in readlines(f)
	num = parse(Int, split(line, " ")[2])
	if startswith(line, "forward")
		x += num
	elseif startswith(line, "up")
		y -= num
	elseif startswith(line, "down")
		y += num
	end
	end
	
	println("a)")
	printfmt("x: {:d}, y: {:d}\n", x, y)
	printfmt("x*y: {:d}\n", x*y)
end


function b()
	f = open("input.txt")

	x, y, aim = 0, 0, 0

	for line in readlines(f)
	num = parse(Int, split(line, " ")[2])
	if startswith(line, "forward")
		x += num
		y += aim * num
	elseif startswith(line, "up")
		aim -= num
	elseif startswith(line, "down")
		aim += num
	end
	end
	
	println("b)")
	printfmt("x: {:d}, y: {:d}\n", x, y)
	printfmt("x*y: {:d}\n", x*y)
end

a()
b()
