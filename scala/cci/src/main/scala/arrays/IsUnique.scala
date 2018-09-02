package arrays

import common.{Solution, Solver}

object IsUnique extends Solution[String, Boolean] {

  solvers.append(Solver("NlogN-sorting", impl1))
  solvers.append(Solver("NlogN-set", impl2))
  solvers.append(Solver("N^2-looping", impl3))
  solvers.append(Solver("N-lookup", impl4))

  private def impl1(str: String): Boolean =
    str.toCharArray.sorted.sliding(2).forall { case Array(a, b) => a != b }

  private def impl2(str: String): Boolean = str.toSet.size == str.size

  private def impl3(str: String): Boolean =
    str.zipWithIndex.forall { case (c, i) => str.substring(i + 1).forall(k => k != c) }

  private def impl4(str: String): Boolean = {
    val chars = Array.fill(Char.MaxValue.toInt)(true)
    str.forall(c => {
      val x = chars(c.toInt)
      chars(c.toInt) = false
      x
    })
  }
}
