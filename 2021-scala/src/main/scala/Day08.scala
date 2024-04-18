import scala.io.Source

object Day08:

  final private val INPUT_DIR = "input/day08/"

  def part1(input: String): Int =
    Source
      .fromFile(INPUT_DIR + input)
      .getLines()
      .map(_.split(" \\| ")(1))
      .flatMap(_.split(" "))
      .count(c => List(2, 3, 4, 7).contains(c.length))

  def part2(input: String): Int =
    Source
      .fromFile(INPUT_DIR + input)
      .getLines()
      .map(x =>
        val sp = x.split(" \\| ")
        (sp(0), sp(1))
      )
      .map(t =>
        val l = t._1.split(" ").sortBy(_.length)
        val s1 = l(0).toSet
        val s7 = l(1).toSet
        val s4 = l(2).toSet
        val s8 = l(9).toSet
        val s2 = l.slice(3, 6).filter(e => (e.toSet & s4).size == 2)(0).toSet
        val s3 = l.slice(3, 6).filter(e => (e.toSet & s1).size == 2)(0).toSet
        val s5 = l.slice(3, 6).filter(e => (e.toSet & s2).size == 3)(0).toSet
        val s6 = l.slice(6, 9).filter(e => (e.toSet & s1).size == 1)(0).toSet
        val s9 = l.slice(6, 9).filter(e => (e.toSet & s4).size == 4)(0).toSet
        val s0 = l.slice(6, 9).filter(e => (e.toSet & s5).size == 4)(0).toSet
        val m = Map(
          s1 -> "1",
          s2 -> "2",
          s3 -> "3",
          s4 -> "4",
          s5 -> "5",
          s6 -> "6",
          s7 -> "7",
          s8 -> "8",
          s9 -> "9",
          s0 -> "0",
        )
        t._2.split(" ").map(x => m(x.toSet)).reduce(_ + _).toInt
      )
      .sum
