# frozen_string_literal: true

class Calculator
  include TablePrintable

  def initialize(field_bound)
    @z = field_bound
  end

  def print_addition_table
    print_table(add_range_legend(calculate_addition_table, '(+)', range))
  end

  def print_multiplication_table
    print_table(add_range_legend(calculate_multiplication_table, '(*)', range))
  end

  def print_inverse_table
    table = [
      calculate_additive_inverse_table,
      calculate_multiplicative_inverse_table
    ]

    print_table(add_table_legend(table, 'Inv.', range, %w[-a a^-1]))
  end

  private

  def calculate_additive_inverse_table
    range.map { |i| (@z - i) % @z }
  end

  def calculate_multiplicative_inverse_table
    range.map do |i|
      range.find { |x| multiply(i, x) == 1 } || 'x'
    end
  end

  def calculate_addition_table
    table { |x, y| (x + y) % @z }
  end

  def calculate_multiplication_table
    table(&method(:multiply))
  end

  def multiply(*args)
    args.reduce(1) { |sum, n| (sum * n) % @z }
  end

  def table
    range.map { |x| range.map { |y| yield x, y } }
  end

  def range
    (0...@z).to_a
  end
end
