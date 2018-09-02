package common

import scala.collection.mutable.ArrayBuffer

case class Input[A](name: String, arg: A)

case class Result[B](time: Double, value: B, solver: String)

case class Solver[A, B](name: String, func: A => B)

trait Solution[A, B] {

  val solvers = ArrayBuffer[Solver[A, B]]()

  def solve(input: Input[A]): B = {
    val solutions = solvers.map(time(_, input))

    // Verify all solvers give same answer
    assert(solutions.map(_.value).distinct.length == 1)

    // Print performance
    println(f"Test case: ${input.name}")
    solutions.sortBy(_.time).foreach(res =>
      println(f"   - ${res.solver} : ${res.time} ms")
    )

    solutions.head.value
  }

  private def time(solver: Solver[A, B], input: Input[A]): Result[B] = {
    var best = Double.MaxValue

    val results = for (i <- 0 to 5) yield {
      val t0 = System.nanoTime()
      val value = solver.func(input.arg)
      val t1 = System.nanoTime()
      best = Math.min(best, (t1 - t0) / 1000000)
      value
    }

    // Verify solver always returns same solution for same input
    assert(results.distinct.length == 1)

    Result(best, results.head, solver.name)
  }
}
