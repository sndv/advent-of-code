import scala.io.Source

object Day05:

  final private val INPUT_DIR = "input/day05/"

  def pPair(p: String): Tuple2[Int, Int] =
    val r = p.split(",").map(_.toInt)
    (r(0), r(1))

  def mplyList[T](l1: List[T], l2: List[T]): List[Tuple2[T, T]] =
    l1.flatMap(e1 => l2.map((e1, _)))

  def linePt(
      start: Tuple2[Int, Int],
      end: Tuple2[Int, Int],
  ): List[Tuple2[Int, Int]] =
    val xList = List(start._1, end._1)
    val yList = List(start._2, end._2)
    mplyList(
      Range(xList.min, xList.max + 1).toList,
      Range(yList.min, yList.max + 1).toList,
    )

  def pRange(start: Int, end: Int): List[Int] =
    if start <= end then Range(start, end + 1).toList
    else Range(end, start + 1).reverse.toList

  def linePtE(
      start: Tuple2[Int, Int],
      end: Tuple2[Int, Int],
  ): List[Tuple2[Int, Int]] =
    if start._1 == end._1 || start._2 == end._2 then
      mplyList(
        pRange(start._1, end._1),
        pRange(start._2, end._2),
      )
    else pRange(start._1, end._1).zip(pRange(start._2, end._2)).toList

  def part1(input: String): Int =
    Source
      .fromFile(INPUT_DIR + input)
      .getLines()
      .map(_.split(" -> "))
      .map(l => (pPair(l(0)), pPair(l(1))))
      .filter(c => c._1._1 == c._2._1 || c._1._2 == c._2._2)
      .flatMap(c => linePt(c._1, c._2))
      .toList
      .groupBy(identity)
      .map(_._2.size)
      .count(_ > 1)

  def part2(input: String): Int =
    Source
      .fromFile(INPUT_DIR + input)
      .getLines()
      .map(_.split(" -> "))
      .map(l => (pPair(l(0)), pPair(l(1))))
      .flatMap(c => linePtE(c._1, c._2))
      .toList
      .groupBy(identity)
      .map(_._2.size)
      .count(_ > 1)
