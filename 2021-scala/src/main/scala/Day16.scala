import scala.io.Source
import java.lang

object Day16:

  final private val INPUT_DIR = "input/day16/"

  final private val HEX_MAP = Map(
    '0' -> "0000",
    '1' -> "0001",
    '2' -> "0010",
    '3' -> "0011",
    '4' -> "0100",
    '5' -> "0101",
    '6' -> "0110",
    '7' -> "0111",
    '8' -> "1000",
    '9' -> "1001",
    'A' -> "1010",
    'B' -> "1011",
    'C' -> "1100",
    'D' -> "1101",
    'E' -> "1110",
    'F' -> "1111",
  )

  def getInput(input: String): String =
    Source
      .fromFile(INPUT_DIR + input)
      .getLines()
      .toList(0)

  def parseLiteralValue(data: String): (String, String) =
    if data(0) == '0' then (data.slice(1, 5), data.splitAt(5)._2)
    else
      val (value, remaining) = parseLiteralValue(data.splitAt(5)._2)
      (data.slice(1, 5) + value, remaining)

  def parseAll(data: String): (List[Long], Long) =
    if data == "" || data.toSet == Set('0') then (List.empty[Long], 0)
    else
      val (value, version, remaining) = parse(data)
      val (values, versionRemaining) = parseAll(remaining)
      (value :: values, versionRemaining + version)

  def boolValue(b: Boolean): Long = if b then 1 else 0

  def parse(data: String): (Long, Long, String) =
    val version = lang.Long.parseLong(data.slice(0, 3), 2)
    val typeId = lang.Long.parseLong(data.slice(3, 6), 2)
    if typeId == 4 then
      val (value, remaining) = parseLiteralValue(
        data.splitAt(6)._2
      )
      (lang.Long.parseLong(value, 2), version, remaining)
    else
      val (valuesDec, versionTotal, remaining) = if data(6) == '1' then
        val subpacketsNum = lang.Long.parseLong(data.slice(7, 18), 2)
        val (values, ver, rem) = Range(0, subpacketsNum.toInt)
          .foldLeft((List.empty[Long], version, data.splitAt(18)._2))((x, _) =>
            val (vl, v, r) = parse(x._3)
            (x._1 :+ vl, x._2 + v, r),
          )
        (values, ver, rem)
      else
        val subpacketsLength = lang.Long.parseLong(data.slice(7, 22), 2)
        val subpacketsData = data.slice(22, 22 + subpacketsLength.toInt)
        val (values, ver) = parseAll(subpacketsData)
        (values, ver + version, data.splitAt(22 + subpacketsLength.toInt)._2)
      val finalValue = typeId match
        case 0 => valuesDec.sum
        case 1 => valuesDec.reduce(_ * _)
        case 2 => valuesDec.min
        case 3 => valuesDec.max
        case 5 => boolValue(valuesDec(0) > valuesDec(1))
        case 6 => boolValue(valuesDec(0) < valuesDec(1))
        case 7 => boolValue(valuesDec(0) == valuesDec(1))
      (finalValue, versionTotal, remaining)

  def part1(input: String): Long =
    parse(getInput(input).map(HEX_MAP(_)).reduce(_ + _))._2

  def part2(input: String): Long =
    parse(getInput(input).map(HEX_MAP(_)).reduce(_ + _))._1
