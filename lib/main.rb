# frozen_string_literal: true

require_relative 'table_printable'
require_relative 'calculator'
require 'colorize'

class Main
  def call
    z = read_field_bound
    calculator = Calculator.new(z)

    puts "\nCalculations tables for Z_#{z}:\n".bold
    puts 'Addition field'
    calculator.print_addition_table

    puts 'Multiplication field'
    calculator.print_multiplication_table

    puts 'Inverse'
    calculator.print_inverse_table
  end

  def read_field_bound
    printf 'Enter field bound: '
    bound = gets.chomp.to_i

    unless bound&.positive?
      puts 'Please enter positive integer'
      exit 1
    end

    bound
  end
end
