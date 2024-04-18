import scala.io.Source
import java.lang
import scala.annotation.tailrec
import scala.collection.immutable.SortedSet

object Day15:

  final private val INPUT_DIR = "input/day15/"

  def inputMap(input: String): List[List[Int]] =
    Source
      .fromFile(INPUT_DIR + input)
      .getLines
      .map(line => line.map(ch => ch.toString.toInt).toList)
      .toList

  def inputMapPart2(input: String): List[List[Int]] =
    val lines = Source
      .fromFile(INPUT_DIR + input)
      .getLines
      .toList
    (0 to 4)
      .flatMap(rr =>
        lines.map(line =>
          (0 to 4)
            .flatMap(cc => line.map(ch => ((ch.toString.toInt + rr + cc - 1) % 9 + 1)).toList)
            .toList
        )
      )
      .toList

  def generateGraph(inputMap: List[List[Int]]): Map[(Int, Int), List[(Int, Int)]] =
    val rows = inputMap.length
    val cols = inputMap(0).length
    (
      for
        r <- 0 until rows
        c <- 0 until cols
      yield (
        (r, c) ->
          List((r, c - 1), (r - 1, c), (r, c + 1), (r + 1, c))
            .filter(ng => ng._1 >= 0 && ng._1 < rows && ng._2 >= 0 && ng._2 < cols)
      )
    ).toMap

  def solve(inMap: List[List[Int]]): Int =
    val graph = generateGraph(inMap)
    val start = (0, 0)
    val target = (inMap.length - 1, inMap(0).length - 1)

    @tailrec def dijkstra(
        visited: Map[(Int, Int), Int],
        toVisit: SortedSet[((Int, Int), Int)],
        toVisitMap: Map[(Int, Int), Int],
    ): Map[(Int, Int), Int] =
      if toVisit.isEmpty then visited
      else
        val lowestDist = toVisit.head
        val (node, dist) = lowestDist
        val toUpdate = graph(node)
          .filter(neigh => !visited.contains(neigh))
          .map(neigh => neigh -> (dist + inMap(neigh._1)(neigh._2)))
          .filter(neigh => neigh._2 < toVisitMap.getOrElse(neigh._1, Int.MaxValue))
        dijkstra(
          visited + (node -> dist),
          toVisit - lowestDist ++ toUpdate,
          toVisitMap ++ toUpdate,
        )

    val visited = dijkstra(
      Map.empty[(Int, Int), Int],
      SortedSet((start, 0))(Ordering[(Int, (Int, Int))].on(el => (el._2, el._1))),
      Map(start -> 0),
    )
    visited(target)

  def part1(input: String): Long =
    solve(inputMap(input))

  def part2(input: String): Long =
    solve(inputMapPart2(input))
