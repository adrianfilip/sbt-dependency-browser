# [Just show me how](#how-do-i-find-out-who-brought-that-dependency-in-my-module)

# Why do I exist?
To filter the data produced by [dependencyTree](https://www.scala-sbt.org/sbt-dependency-graph/) and find out how that pesky transitive library ended up in my module.
I just want to know for dependencies X,Y,Z what dependencies bring it in my project.

# Doesn't [dependencyBrowseGraph](https://www.scala-sbt.org/sbt-dependency-graph/) solve that for me?
Not always apparently, it fails with stackoverflow sometimes.

# Why not ide or some text editor?
Text editor wise it's not great because you lose the structure.
IDEs are not great with 10Mb files, tend to freeze up (including nvim :( )

# How do I find out who brought that dependency in my module?
Use the [dependencyTree from sbt-dependency-graph plugin](https://www.baeldung.com/scala/sbt-dependency-tree) to export all data.

```
sbt dependencyTree > dependencyTreeExport.txt
```

Then filter that with sbt-dependency-browser by targetting the dependencies causing your build to fail.
```
cargo run dependencyTreeExport.txt akka-actor
```

The result for a single parameter:
```
com.myproject:mytest_3:0.0.0-166-cc699f8c
 | com.myproject:functional-tests_3:0.0.0-166-cc699f8c
 |  | com.myproject.user:interp-ce3_3:0.0.0+2581-e524..
 |  |  | com.myproject.user:interp-core-ce3_3:0.0.0+2581-e524884d
 |  |  |  | com.typesafe.akka:akka-actor_3:2.6.21
 |  |  |  | com.typesafe.play:play-ahc-ws-standalone_3:2.2.4
 |  |  |  |  | com.typesafe.play:play-ws-standalone_3:2.2.4
 |  |  |  |  |  | com.typesafe.akka:akka-stream_3:2.6.20
 |  |  |  |  |  |  | com.typesafe.akka:akka-actor_3:2.6.20 (evicted by: 2.6.21)
 |  |  |  |  |  |  | com.typesafe.akka:akka-actor_3:2.6.21
 |  |  |  | com.typesafe.play:play-ws-standalone_3:2.2.4
 |  |  |  |  | com.typesafe.akka:akka-stream_3:2.6.20
 |  |  |  |  |  | com.typesafe.akka:akka-actor_3:2.6.20 (evicted by: 2.6.21)
 |  |  |  |  |  | com.typesafe.akka:akka-actor_3:2.6.21
 | io.gatling:gatling-core:3.9.5 [S]
 |  | com.typesafe.akka:akka-actor_2.13:2.6.20 [S]
 |  | com.typesafe.akka:akka-slf4j_2.13:2.6.20 [S]
 |  |  | com.typesafe.akka:akka-actor_2.13:2.6.20 [S]
 | io.gatling:gatling-http:3.9.5 [S]
 |  | io.gatling:gatling-core:3.9.5 [S]
 |  |  | com.typesafe.akka:akka-slf4j_2.13:2.6.20 [S]
 |  |  |  | com.typesafe.akka:akka-actor_2.13:2.6.20 [S]
```

```
#or if you want to filter for multiple parameters
cargo run dependencyTreeExport.txt akka-actor scala-parser-combinators
```

Or if you want to filter with multiple parameters you should see something like:
```
com.myproject:mytest_3:0.0.0-166-cc699f8c
 | com.myproject:functional-tests_3:0.0.0-166-cc699f8c
 |  | com.myproject.user:interp-ce3_3:0.0.0+2581-e524..
 |  |  | com.myproject.user:interp-core-ce3_3:0.0.0+2581-e524884d
 |  |  |  | com.typesafe.akka:akka-actor_3:2.6.21
 |  |  |  | com.typesafe.play:play-ahc-ws-standalone_3:2.2.4
 |  |  |  |  | com.typesafe.play:cachecontrol_3:2.3.1
 |  |  |  |  |  | org.scala-lang.modules:scala-parser-combinators_3:2.3.0
 |  |  |  |  | com.typesafe.play:play-ws-standalone_3:2.2.4
 |  |  |  |  |  | com.typesafe.akka:akka-stream_3:2.6.20
 |  |  |  |  |  |  | com.typesafe.akka:akka-actor_3:2.6.20 (evicted by: 2.6.21)
 |  |  |  |  |  |  | com.typesafe.akka:akka-actor_3:2.6.21
 |  |  |  | com.typesafe.play:play-ws-standalone_3:2.2.4
 |  |  |  |  | com.typesafe.akka:akka-stream_3:2.6.20
 |  |  |  |  |  | com.typesafe.akka:akka-actor_3:2.6.20 (evicted by: 2.6.21)
 |  |  |  |  |  | com.typesafe.akka:akka-actor_3:2.6.21
 | io.gatling:gatling-core:3.9.5 [S]
 |  | com.typesafe.akka:akka-actor_2.13:2.6.20 [S]
 |  | com.typesafe.akka:akka-slf4j_2.13:2.6.20 [S]
 |  |  | com.typesafe.akka:akka-actor_2.13:2.6.20 [S]
 |  | io.gatling:gatling-jsonpath:3.9.5 [S]
 |  |  | org.scala-lang.modules:scala-parser-combinators_2.13:2.3.0 [S]
 |  | org.scala-lang.modules:scala-parser-combinators_2.13:2.3.0 [S]
 | io.gatling:gatling-http:3.9.5 [S]
 |  | io.gatling:gatling-core:3.9.5 [S]
 |  |  | com.typesafe.akka:akka-actor_2.13:2.6.20 [S]
 |  |  | com.typesafe.akka:akka-slf4j_2.13:2.6.20 [S]
 |  |  |  | com.typesafe.akka:akka-actor_2.13:2.6.20 [S]
 |  |  | io.gatling:gatling-jsonpath:3.9.5 [S]
 |  |  |  | org.scala-lang.modules:scala-parser-combinators_2.13:2.3.0 [S]
 |  |  | org.scala-lang.modules:scala-parser-combinators_2.13:2.3.0 [S]

```

# Why is this written in Rust and not Scala?
Next question please.
