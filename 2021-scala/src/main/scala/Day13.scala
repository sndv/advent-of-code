import scala.io.Source

object Day13:

  final private val INPUT_DIR = "input/day13/"

  def fold(
      dots: Set[(Int, Int)],
      foldDir: String,
      foldIdx: Int,
  ): Set[(Int, Int)] =
    if foldDir == "x" then
      dots.map(d =>
        if d._1 > foldIdx then (foldIdx - (d._1 - foldIdx), d._2)
        else d,
      )
    else if foldDir == "y" then
      dots.map(d =>
        if d._2 > foldIdx then (d._1, foldIdx - (d._2 - foldIdx))
        else d,
      )
    else throw new RuntimeException

  def parseInput(input: String): (Set[(Int, Int)], List[(String, Int)]) =
    val lines = Source
      .fromFile(INPUT_DIR + input)
      .getLines()
      .toList
    val dots = lines
      .filter(_.contains(","))
      .map(_.split(","))
      .map(e => (e(0).toInt, e(1).toInt))
      .toSet
    val folds = lines
      .filter(_.contains("="))
      .map(_.split(" ")(2))
      .map(_.split("="))
      .map(e => (e(0), e(1).toInt))
    (dots, folds)

  def part1(input: String): Int =
    val (dots, folds) = parseInput(input)
    fold(dots, folds(0)._1, folds(0)._2).size

  def part2(input: String): String =
    val (dots, folds) = parseInput(input)
    val codeDots = folds.foldLeft(dots)((d, f) => fold(d, f._1, f._2))
    val code = Range(0, codeDots.map(_._2).max + 1)
      .map(y =>
        Range(0, codeDots.map(_._1).max + 1)
          .map(x => if codeDots.contains((x, y)) then "#" else " ")
          .reduce(_ + _),
      )
      .reduce(_ + "\n" + _)
    "\n" + code + "\n"
