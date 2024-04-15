import scala.io.Source
import scala.annotation.tailrec

object Day09:

  final private val INPUT_DIR = "input/day09/"

  def cnvInt(c: Char): Int =
    if c == 'a' then 10
    else c.toString.toInt

  def part1(input: String): Int =
    val in1 = Source
      .fromFile(INPUT_DIR + input)
      .getLines()
      .toList
      .map("a" + _ + "a")
    val aStr = "a" * in1(0).length
    val in2 = List(aStr) ++ in1 ++ List(aStr)

    in2
      .sliding(3)
      .toList
      .map(g =>
        g(0)
          .sliding(3)
          .toList
          .lazyZip(g(1).sliding(3).toList)
          .lazyZip(g(2).sliding(3).toList)
          .map((el1, el2, el3) =>
            val t = cnvInt(el2(1))
            if t < cnvInt(el1(1))
              && t < cnvInt(el2(0))
              && t < cnvInt(el2(2))
              && t < cnvInt(el3(1))
            then t + 1
            else 0,
          )
          .sum,
      )
      .sum

  def doFindBasin(
      baseCheck: Boolean,
      target: (Int, Int),
      map: List[String],
      start: (Int, Int),
      currBasin: Set[(Int, Int)] = Set.empty,
  ): Set[(Int, Int)] =
    if baseCheck
      && !currBasin.contains(target)
      && map(target._1)(target._2) != '9'
    then findBasin(map, target, currBasin + start)
    else currBasin

  def findBasin(
      map: List[String],
      start: (Int, Int),
      currBasin: Set[(Int, Int)] = Set.empty,
  ): Set[(Int, Int)] =
    val up =
      doFindBasin(start._1 > 0, (start._1 - 1, start._2), map, start, currBasin)
    val left =
      doFindBasin(start._2 > 0, (start._1, start._2 - 1), map, start, up)
    val right = doFindBasin(
      start._2 + 1 < map(0).length,
      (start._1, start._2 + 1),
      map,
      start,
      left,
    )
    doFindBasin(
      start._1 + 1 < map.length,
      (start._1 + 1, start._2),
      map,
      start,
      right,
    ) + start

  @tailrec
  def findAllBasins(
      map: List[String],
      basins: List[Set[(Int, Int)]] = List.empty,
  ): List[Set[(Int, Int)]] =
    val joinedMap = map
      .reduce(_ + _)
      .map(e => if e == '9' then '9' else '0')
      .mkString
    val start = joinedMap.indexOf('0')
    if start == -1 then basins
    else {
      val basin =
        findBasin(map, (start / map(0).length, start % map(0).length))
      findAllBasins(
        map.zipWithIndex.map(r =>
          r._1.zipWithIndex
            .map(c => if basin.contains((r._2, c._2)) then '9' else c._1)
            .mkString,
        ),
        basins :+ basin,
      )
    }

  def part2(input: String): Int =
    findAllBasins(
      Source.fromFile(INPUT_DIR + input).getLines().toList,
    ).map(_.size).sorted.reverse.splitAt(3)._1.foldLeft(1)(_ * _)
