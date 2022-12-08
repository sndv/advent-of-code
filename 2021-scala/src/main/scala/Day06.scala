import scala.io.Source
import scala.annotation.tailrec

object Day06 {

  final private val INPUT_DIR = "input/day06/"

  def initialFish(input: String) = Source
    .fromFile(INPUT_DIR + input)
    .getLines()
    .toList(0)
    .split(",")
    .map(_.toInt)
    .toList

  @tailrec
  def grow(fish: List[Int], days: Int): List[Int] =
    if (days == 0) fish
    else
      grow(
        fish.flatMap(f => if (f == 0) List(6, 8) else List(f - 1)),
        days - 1
      )

  def part1(input: String): Int = {
    grow(initialFish(input), 80).length
  }

  def part2slow(input: String): BigInt = {
    val daysTotal = 256
    val daysStep1 = 100
    val step1fish = grow(initialFish(input), daysStep1)
    val step1time = System.currentTimeMillis()
    val m = Range(0, 9).map(n => grow(List(n), daysTotal - daysStep1).length)
    step1fish.map(e => BigInt(m(e))).sum
  }

  @tailrec
  def grow2(fish: Map[Int, BigInt], days: Int): Map[Int, BigInt] =
    if (days == 0) fish
    else {
      grow2(
        fish.toList
          .flatMap(e =>
            if (e._1 == 0) List((6, e._2), (8, e._2))
            else List((e._1 - 1, e._2))
          )
          .groupBy(_._1)
          .mapValues(_.reduce((t1, t2) => (t1._1, t1._2 + t2._2)))
          .map(_._2)
          .toList
          .toMap,
        days - 1
      )
    }

  def part2(input: String): BigInt = {
    grow2(
      initialFish(input)
        .groupBy(identity)
        .mapValues(v => BigInt(v.length))
        .toMap,
      256
    ).values.sum
  }
}
