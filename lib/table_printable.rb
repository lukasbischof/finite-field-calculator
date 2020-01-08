# frozen_string_literal: true

require 'terminal-table'
require 'colorize'

module TablePrintable
  protected

  def add_range_legend(table, title, range)
    add_table_legend(table, title, range, range)
  end

  def add_table_legend(table, title, x_legend, y_legend)
    [format_legend([title, *x_legend])] + table.map.with_index { |row, i| [format_legend(y_legend[i]), *row] }
  end

  def format_legend(legend)
    return format_legend([legend]).first unless legend.is_a? Array

    legend.map(&(proc(&:to_s) >> proc(&:blue) >> proc(&:bold)))
  end

  def print_table(table)
    puts Terminal::Table.new(rows: table, style: { all_separators: true })
  end
end
