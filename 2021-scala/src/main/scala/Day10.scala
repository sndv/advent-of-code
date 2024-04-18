import scala.io.Source
import scala.annotation.tailrec

object Day10:

  final private val INPUT_DIR = "input/day10/"

  final val PAIRS = List("()", "[]", "{}", "<>")
  final val CLOSE_MAP = Map('(' -> ')', '[' -> ']', '{' -> '}', '<' -> '>')
  final val VALUE_MAP_A = Map(')' -> 3, ']' -> 57, '}' -> 1197, '>' -> 25137)
  final val VALUE_MAP_B = Map(')' -> 1, ']' -> 2, '}' -> 3, '>' -> 4)

  @tailrec
  def cleanOkBr(nav: String): String =
    if !PAIRS.map(nav.contains(_)).reduce(_ || _) then nav
    else
      cleanOkBr(
        nav
          .replace("()", "")
          .replace("[]", "")
          .replace("{}", "")
          .replace("<>", "")
      )

  def part1(input: String): Int =
    Source
      .fromFile(INPUT_DIR + input)
      .getLines()
      .map(cleanOkBr(_))
      .map(
        _.flatMap(x => if VALUE_MAP_A.contains(x) then Some(x) else None).mkString
      )
      .filter(_ != "")
      .map(x => VALUE_MAP_A(x(0)))
      .sum

  def part2(input: String): BigInt =
    val scores = Source
      .fromFile(INPUT_DIR + input)
      .getLines()
      .map(cleanOkBr(_))
      .filter(_ != "")
      .filter(_.map(!VALUE_MAP_A.contains(_)).reduce(_ && _))
      .map(_.reverse.map(CLOSE_MAP(_)).mkString)
      .map(_.foldLeft(BigInt(0))((s, b) => s * 5 + VALUE_MAP_B(b)))
      .toList
      .sorted
    scores(scores.length / 2)
