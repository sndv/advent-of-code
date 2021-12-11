import scala.io.Source
import scala.math.pow
import scala.annotation.tailrec

object Day03 {

  final private val INPUT_DIR = "input/day03/"

  def part1(input: String): Int = {
    val gammaBin = Source
      .fromFile(INPUT_DIR + input)
      .getLines()
      .map(_.toList.map(_.toString.toInt * 2 - 1))
      .reduce(_.zip(_).map(e => e._1 + e._2))
      .map(e => if (e < 0) "0" else "1")
      .mkString
    val gamma = Integer.parseInt(gammaBin, 2)
    gamma * (pow(2, gammaBin.length).toInt - gamma - 1)
  }

  def part2(input: String): Int = {

    def oposite(ch: Char): Char = if (ch == '0') '1' else '0'

    @tailrec
    def rating(numbers: List[String], default: Char, pos: Int = 0): String = {
      if (numbers.length == 1) {
        numbers(0)
      } else {
        val target = numbers
          .map(_.toList(pos).toString.toInt * 2 - 1)
          .reduce(_ + _) match {
          // more 0s -> if default is 1 then 0, if default is 0 then 1
          case n if n < 0 => oposite(default)
          // more 1s -> if default is 1 then 1, if default is 0 then 0
          case n if n >= 0 => default
        }
        rating(numbers.filter(_(pos) == target), default, pos + 1)
      }
    }

    val lines = Source.fromFile(INPUT_DIR + input).getLines().toList
    (Integer.parseInt(rating(lines, '1'), 2)
      * Integer.parseInt(rating(lines, '0'), 2))
  }
}
