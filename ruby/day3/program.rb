def engine_parts_from(line_idx, line)
  lines_numbers = []
  lines_gears = []
  line_chars = line.chars
  cursor_at = 0

  line_chars.each_with_index do |char, position|
    next if cursor_at > position

    num = ''
    cursor_at = position

    case char
    when '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
      num << char

      loop do
        cursor_at += 1

        break if cursor_at >= line_chars.size || !line_chars[cursor_at].match?(/[0-9]/)

        num << line_chars[cursor_at]
      end

      lines_numbers << { line: line_idx, position: position.to_i, value: num.to_i, length: num.size }
    when '*'
      lines_gears << { line: line_idx, position: position.to_i }
    end
  end

  [lines_numbers, lines_gears]
end

def char_at_position(position, content)
  content[position[0]][position[1]]
rescue StandardError
  nil
end

input = File.read('input.txt')
lines = input.split("\n")
lines_numbers = []
lines_gears = []

lines.each_with_index do |line, position|
  n, g = engine_parts_from(position, line)

  lines_numbers << n
  lines_gears << g
end

result_1 = lines_numbers.map do |numbers_per_line|
  numbers_per_line.reduce(0) do |acc, num|
    neighbours = []
    my_line_prev_pos = [num[:line], num[:position] - 1]
    my_line_next_pos = [num[:line], num[:position] + num[:length]]
    prev_line_my_pos = [num[:line] - 1, num[:position]]
    prev_line_prev_pos = [num[:line] - 1, num[:position] - 1]
    prev_line_next_pos = [num[:line] - 1, num[:position] + num[:length]]
    next_line_my_pos = [num[:line] + 1, num[:position]]
    next_line_prev_pos = [num[:line] + 1, num[:position] - 1]
    next_line_next_pos = [num[:line] + 1, num[:position] + num[:length]]

    neighbours << char_at_position(my_line_prev_pos, lines)
    neighbours << char_at_position(my_line_next_pos, lines)
    (prev_line_prev_pos[1]..prev_line_next_pos[1]).each do |pos|
      neighbours << char_at_position([prev_line_my_pos[0], pos], lines)
    end
    (next_line_prev_pos[1]..next_line_next_pos[1]).each do |pos|
      neighbours << char_at_position([next_line_my_pos[0], pos], lines)
    end

    acc += num[:value] if neighbours.compact.any? { |n| !n.to_s.match?(/[0-9]/) && n != '.' }
    acc
  end
end.sum

result_2 = lines_gears.select(&:any?).map do |line_gears|
  line_gears.reduce(0) do |acc, gear|
    rows_to_search = [gear[:line].to_i - 1, gear[:line], gear[:line].to_i + 1]

    found_numbers = []
    rows_to_search.each do |row|
      lines_numbers[row].each do |num|
        found_numbers << num if (num[:position] - 1..num[:position] + num[:length]).to_a.include?(gear[:position])
      end
    end

    acc += found_numbers[0][:value] * found_numbers[1][:value] if found_numbers.count == 2
    acc
  end
end.sum

puts result_1
puts result_2
