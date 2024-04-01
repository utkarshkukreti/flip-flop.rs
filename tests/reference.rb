count_1, left_1, right_1 = 0, 0, 0
count_2, left_2, right_2 = 0, 0, 0
count_3, left_3, right_3 = 0, 0, 0
count_4, left_4, right_4 = 0, 0, 0

0.upto(20).each do |i|
  if (left_1 += 1; i == 5)..(right_1 += 1; i == 10)
    count_1 += 1
  end
  if (left_2 += 1; i == 5)...(right_2 += 1; i == 10)
    count_2 += 1
  end
  if (left_3 += 1; i % 2 == 0)..(right_3 += 1; i % 4 == 0)
    count_3 += 1
  end
  if (left_4 += 1; i % 2 == 0)...(right_4 += 1; i % 4 == 0)
    count_4 += 1
  end
end

p [count_1, left_1, right_1]
p [count_2, left_2, right_2]
p [count_3, left_3, right_3]
p [count_4, left_4, right_4]
