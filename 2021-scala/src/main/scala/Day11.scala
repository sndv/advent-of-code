import scala.io.Source
import scala.annotation.tailrec

object Day11:

  final private val INPUT_DIR = "input/day11/"

  def neighbours(idx: Int, rows: Int, cols: Int): List[Int] =
    val left = idx % cols != 0
    val right = idx % cols != cols - 1
    val up = idx >= cols
    val down = idx / cols < rows - 1
    List(
      (idx - cols, up),
      (idx + cols, down),
      (idx - 1, left),
      (idx + 1, right),
      (idx - cols - 1, up && left),
      (idx - cols + 1, up && right),
      (idx + cols - 1, down && left),
      (idx + cols + 1, down && right),
    ).flatMap(x => if x._2 then Some(x._1) else None)

  @tailrec
  def flashes(
      octopi: List[Int],
      rows: Int,
      cols: Int,
      fln: Int = 0,
  ): (List[Int], Int) =
    val fl = octopi.map(o => if o > 9 then 1 else 0)
    val newOctopi = octopi.map(o => if o > 9 then -100 else o)
    if fl.sum > 0 then
      flashes(
        newOctopi.zipWithIndex.map(o => o._1 + neighbours(o._2, rows, cols).map(fl(_)).sum),
        rows,
        cols,
        fln + fl.sum,
      )
    else (newOctopi, fln)

  def simulateCycle(
      octopi: List[Int],
      rows: Int,
      cols: Int,
  ): (List[Int], Int) =
    val (newOctopi, fln) = flashes(octopi.map(_ + 1), rows, cols)
    (newOctopi.map(o => if o < 0 then 0 else o), fln)

  @tailrec
  def passCycles(
      octopi: List[Int],
      steps: Int,
      rows: Int,
      cols: Int,
      total: Int = 0,
  ): Int =
    if steps == 0 then total
    else {
      val (newOctopi, fln) = simulateCycle(octopi, rows, cols)
      passCycles(newOctopi, steps - 1, rows, cols, total + fln)
    }

  @tailrec
  def passUntilSync(
      octopi: List[Int],
      rows: Int,
      cols: Int,
      step: Int = 1,
  ): Int =
    val (newOctopi, _) = simulateCycle(octopi, rows, cols)
    if newOctopi.sum == 0 then step
    else passUntilSync(newOctopi, rows, cols, step + 1)

  def part1(input: String): Int =
    val data = Source
      .fromFile(INPUT_DIR + input)
      .getLines()
      .toList
    val octopi = data.map(_.map(_.toString.toInt).toList).reduce(_ ++ _)

    passCycles(octopi, 100, data.length, data(0).length)

  def part2(input: String): Int =
    val data = Source
      .fromFile(INPUT_DIR + input)
      .getLines()
      .toList
    val octopi = data.map(_.map(_.toString.toInt).toList).reduce(_ ++ _)

    passUntilSync(octopi, data.length, data(0).length)
