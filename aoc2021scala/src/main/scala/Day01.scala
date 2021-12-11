import scala.io.Source

object Day01 {

  final private val INPUT_DIR = "input/day01/"

  def part1(input: String): Int = {
    Source
      .fromFile(INPUT_DIR + input)
      .getLines()
      .sliding(2)
      .map(s => s(0).toInt < s(1).toInt)
      .count(identity)
  }

  def part2(input: String): Int = {
    Source
      .fromFile(INPUT_DIR + input)
      .getLines()
      .sliding(4)
      .map(s => s(0).toInt < s(3).toInt)
      .count(identity)
  }
}
