import scala.io.Source
import scala.collection.immutable

object Day07:

  final private val INPUT_DIR = "input/day07/"

  def median(l: List[Int]): List[Int] =
    l.sorted match
      case l if l.length % 2 == 1 => List(l((l.length - 1) / 2))
      case l =>
        val m = ((l(l.length / 2) + l(l.length / 2 - 1)) / 2)
        List(m, m + 1)

  def average(l: List[Int]): List[Int] =
    val s = l.sum
    val a = s / l.length
    if s % l.length == 0 then List(a)
    else List(a, a + 1)

  def crabsPos(input: String) = Source
    .fromFile(INPUT_DIR + input)
    .getLines()
    .toList(0)
    .split(",")
    .map(_.toInt)
    .toList

  def part1(input: String): Int =
    val crabs = crabsPos(input)
    val target = median(crabs)
    median(crabs).map(t => crabs.map(c => (c - t).abs).sum).min

  def part2(input: String): Int =
    val crabs = crabsPos(input)
    average(crabs)
      .map(t =>
        crabs
          .map(c =>
            val steps = (c - t).abs
            ((steps + 1) * steps) / 2
          )
          .sum
      )
      .min
