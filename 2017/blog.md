# Reflections on Advent of Code

This year I was introduced to Advent of Code by a coworker, who also made the offhand comment that some people try to do it in 25 different languages. That sounded like the right amount of challenge for an over-achiever like me, so I decided to attempt it, not knowing at all what the difficulty of the daily challenges would look like. My assumption was that the challenges would start out fairly easy, and then gradually get harder. As a result, I sorted my programming languages by the degree I felt comfortable with them, as well as how good they would be at solving a problem. The programming languages I chose were a mix of languages I knew, languages that interested me, languages that had been recommended to me, and top programming languages. I tried to avoid languages that shared a standard library (all of the .NET languages), unless they really behaved differently.

## General thoughts

I thought that the puzzles were all pretty well designed. The first few days were a bit simplistic, but that turned out to be ok (as I will get to in my in depth comments). I liked that some questions couldn't be brute-forced (turns out 10 billion is a really big number), but I agree with other commenters that maybe some of the problems that I could brute-force should've been designed in a way that I couldn't. What did trip me up a few times (in a good way) was that I solved part 1 in the most straight forward way possible, that just couldn't be extended into part 2. Overall I had a great time, and became an Advent of Code supporter, mostly for that sweet AoC++ flair. I do wish that the release time was a bit earlier for those of us in EST, but I do understand why things are the way they are.

As for "learning" 8+ languages from scratch, and struggling with a less-than-ideal understanding of many more, I can say I was really happy when I got to the last week of challenges and didn't have to think as hard anymore. As others have said, I really gained a "drive by" understanding of the languages I didn't know, not much more than a feel for their semantics. I'll comment more on the languages as I go through the problems in detail, but for now I'll say that Scala was the most intriguing language, followed by Kotlin, with Go being the most disappointing language.

I used docker containers for compiling and running my code, which made my life a lot easier. I don't know if I would do everyday development in docker containers, my VS Code extensions didn't always like not being able to access the tools, but it was a breeze to run most languages. The most difficult to run were Common Lisp, probably for obvious reasons, and OCaml, because I needed to install additional packages. Most of my solutions didn't require me to include any libraries, so that worked out well.

## Day by Day comments

### Day 1 – Bash

Good thing I had an easy problem for this language. I used pure Bash, no Unix tools. Not much else to say.

### Day 2 – x86

Definitely went from easiest to hardest here. I finished part 1 at 8:36 PM and part 2 at 11:03 PM, and that was with only about a 3 hour break during the afternoon. Using assembly was challenging. If I hadn't written a compiler in my last semester and therefore knew about stack management and calling conventions, I probably would've been done for. At one point I said to myself, "I don't know why I'm doing `[rbp+16]` to get the first argument from the stack, but I know it's right so I'm going to keep doing it". Unfortunately I later second-guessed this assertion, and ended up wasting a bunch of time messing around with stacks only to arrive back at my original code.

I had never used 64-bit assembly before and it introduced a few quirks that I wasn't expecting, besides prefixing everything with 'r'. All my muscle memory was for working with multiples of 4, and having 8s all over the place was weird. In hindsight, instead of messing around with the stack for parameters I should've just used all the extra registers a 64-bit architecture gives you, but I didn't know about that until much later on in my solution. I still don't really know what the nesting level of `enter` does, but it was handy. I also wish I could've figured out some of nasm's macros, like `%local`, but I didn't have enough time to really dive into them.

Writing routines to parse and print numbers took the bulk of my time here. That was where I figured out calling convention stuff. I didn't use the C standard library because that didn’t seem right, so I just used syscalls directly. Unfortunately I solved part 1 with a linear scan, so I had to figure out how to read the numbers into an array for part 2. The actual implementation of the algorithms to solve the problem was fairly easy, compared to the set up for the utility functions.

Working in assembly resulted in some nasty, and hard to track down, bugs. A few that I encountered were; referencing things relative to the stack pointer instead of the base pointer (and then changing the stack pointer), reusing registers in one other place and not realizing I had to push them, and not using `movzx` when working with single bytes. Overall, assembly is an interesting way to think about problems, but there's too many things you have to keep track of in your head.

### Day 3 – Matlab

Not much to say here, other than I wish I had saved Matlab for Day 21. I also still felt like part 2 should be solved with an equation, but I wasn't dedicated enough to figure that out.

### Day 4 – R

Also not much to say here. I expected R to feel more functional, but with the way functions composed it still felt more imperative.

### Day 5 – Perl

This took almost 4 minutes to run. I'm disappointed in you Perl. Maybe I could've optimized this somehow, but my non-trivial amount research didn't turn up anything. Also, I hate sigils. I don't get what people see in them, and the number of times I forgot the $ on a variable, only to have the compiler remind me to include was very frustrating. If the compiler can figure out where I forgot a $, why do I have to include it? `my` for variable declarations? Why? This is the start of a theme that I will elabortate on, sometimes languages are different for no apparent reason. I think there is a lot of value in having certain things be consistent across languages. My final complaint with perl is that remembering semi-colons in scripting languages is hard.

### Day 6 – PHP

This language is the ugly love-child of C and Perl. I can see how PHP might be fast for creating small websites, but I don't think I'd want to do anything robust in it. I also don't like the idea of passing functions as parameters by a string representing their name. That seems like laziness in language design. When it came to solving the actual problem, I got caught by accidentally using the lexigraphical ordering of strings, instead their integer values. Types are nice.

### Day 7 – Julia

I certainly didn't use this language enough to do it justice. It seems interesting, an array-focused, Python-like language. Using `nothing` instead of `None` like python (or `null`) was questionable, as well as starting array indexing at `1`, but that's just me nit-picking. I basically just wrote it as Python for this problem, which worked out for me, but I plan on coming back to Julia at some point.

### Day 8 – Kotlin

Kotlin seems really interesting. I had a rough time setting up it, since IntelliJ wanted me to make a whole project for it, but I eventually got there. Having a script file extension was nice, although it was no easier to run using an IDE, which I found annoying. Kotlin's treatment of nulls is great, with special operators that force safety. The semantics are similar to Groovy, so I can see it competing in the JVM scripting niche, or as a more lightweight Java. I'd use it again for a small to medium sized project.

### Day 9 – OCaml

As much as I want to like this language I can't. It's nothing to do with language design, much of that has been successfully carried forward to F#. It's the community and environment. I was lucky to know that I would have to install an alternative standard library before going into this (although I tried not doing that and failed), but a newbie to a language shouldn't have to know that. There are at least 3 competing alternatives to the standard library, which leads to a language that is not easy to pick up. Any beginner-level question will need to be answered once for each standard library. I found output of the documentation generator for OCaml is also very difficult to read which made using these various standard libraries even more difficult.

### Day 10 – Elixir

I found this language the most difficult to pick up. I don't know if it was how the piping operator works, or the semantics for defining and calling lambdas, but writing code in this language was work. The code is understandable once it's written, but I had a lot of issues getting to that point. Elixir definitely has potential, but I would pick Scala for a similar use case.

### Day 11 – Groovy

Nothing interesting to say here. I've used Groovy for automation and scripting before, so it wasn't new to me. I was really needing a break from learning new languages at this point; it's mentally draining to start your day by learning a new language.

### Day 12 – Go

So, this is my most disappointing language. I haven't done much reading about Go, so I'm sure there are rebuttals to anything I complain about. But I can say that going into it blind, only hearing comments like "Go is great", it doesn't live up. I found the documentation rather poor, searching "map" on golang.org gives

```
Results for query: map
    Did you mean: Map MAP

6887 textual occurrences

/AUTHORS:                         2    340 632
/CONTRIBUTORS:                    2    465 843
/doc/articles/go_command.html:    1    118
...
```

And proceeds to tell me nothing about how to use a map in Go. From some cursory reading of SO answers I got "read the language specification section on maps, it's quite readable." This isn't my idea of how to introduce somebody to a language feature. I found Go rather clunky, and lacking the expressiveness I would expect for a language that is popular right now. Operations like finding a single key in a map needed to be written out using a for loop and breaking after the first iteration. `:=` for declarations tripped me up when I accidently shadowed a variable trying to assign to it in an inner block. I'm sure a linter could catch that, but a language like Rust would warn you of that at compile time by default.

It doesn't immediately jump out to me what kind of use case Go is trying to address. I know that it is good for concurrency, but I'm not sure when I'd use it. For low-level programming Rust seems much better, and for higher-level programming I'd prefer a language that is more expressive. Maybe I just need to try writing something a little less trivial in Go.

### Day 13 – Scala

I liked Scala a lot. It was easy to apply my previous experience with functional programming. It has a fairly lightweight type system, and error messages are easy to understand. I don't quite get the semantics of when you can omit parentheses, but I'm sure that would come with time. I hope to use this language again, either for web programming with Scala.JS or server-side. Functional programming also leant itself well to this problem, as immutable state covered the backtracking aspect of part 2. The naive approach ended up being linear in the number of states and the delay.

### Day 14 – Clojure

I really enjoy writing Clojure, but I had forgotten how obtuse the error messages were. At one point I was passing arguments in the wrong order and I took me forever to figure out what the error message was telling me. The incredibly long stack traces that include the interpreter's code don't help either. Debugging is also somewhat challenging. I still think that Clojure is my favourite Lisp dialect, but I don't think I could write larger programs with it.

### Day 15 – Common Lisp

This was a bad decision. I should've picked Racket or Scheme, something with more widespread use. Even finding a compiler was non-trivial, but I did find one with a docker container. Common Lisp was less similar to Clojure than I was expecting, but I figured it out pretty quickly since I didn't need to do anything very complicated. I would've liked to solve this problem with lazy sequences or generators, but I don't know if that's possible with Common Lisp. A simple reduce worked just fine though.

### Day 16 – Python

I had the opportunity to stay up until midnight on day 16 and 17 without being punished the next morning, so I tried my best two languages, hoping to make it on the leaderboard. Unfortunately, I did not get the trick of this problem for quite a while. I thought maybe Python was too slow, so I actually tried re-writing it in Rust while I was waiting for one attempt to terminate. Turns out even `1...10000000` times out in Rust. When my roommate told me he figured out the shortcut at 1:30 AM I was getting a little frustrated. Being up that late probably wasn't helping me think any better either. I eventually figured it out around 2:00 AM, but it was a bitter victory.

### Day 17 – Javascript

Day 17 convinced me that Javascript is definitely my best interview language. It probably helps that I write in it every day. Unfortunately, some failed premature optimization and forgetting how `splice` works put me at 105th, but I still felt I did well.

### Day 18 – F#

F# is a great language, 'nuff said. I got stuck for quite a while on this one, and ended up browsing [/r/adventofcode](https://www.reddit.com/r/adventofcode) to see what other people had gotten stuck on.  It seemed like everyone was getting stuck on misparsing the `jnz` instruction, which I had totally accounted for. I explained as much to my roommate, realizing in the process that I had _not_ in fact accounted for that case...

### Day 19 – Ruby

This puzzle was cute, Ruby worked well.

### Day 20 – C

I am proud to say that I still know linked list operations like the back of my hand. Not a very useful skill for my day to day, but if it ever comes up again, I know I'm ready!

### Day 21 – C#

I started this one in Haskell and quickly abandoned that idea. I wish I had Matlab available for this one, but I made do with C#. I wasn't aware of rectangular arrays before this. A rectangular array looks like this

```c#
int[,] array = {
    {1, 2, 3},
    {4, 5, 6},
};
int b = array[0, 1];
// Get the upper bound of the 0th dimension
int n = array.GetUpperBound(0);
assert(a[n, 0] == 4);
```

I don't think they made my life easier since you can't extract out single dimensions, but it was a cool language feature to learn about.

### Day 22 – Haskell

I really like Haskell, so it was fun to come back to it. I wish I had more opportunities to use it in general.

### Day 23 – Rust

This was a really interesting puzzle, although like many others I assumed I needed to write a program to solve this puzzle in the general case. I couldn't see a way to implement an optimization myself, so I decided to be cheeky and instead write a Rust program to compile this pseudo-assembly into x86, and then pass it to `gcc –O3` for optimization. I initially worried about the possibility of dynamic offsets for jumps, but that didn't occur anywhere, so the compiler wasn't too hard to write. I just printed a label for each source line, and evaluated the jumps at compile time. When the output from gcc failed to terminate I knew I just had to do it the manual way. It was still enjoyable, and fairly easy after my previous practice with assembly.

### Day 24 – C++

Oh C++. I immediately felt like I was writing a school assignment, and also felt rebellious for my usage of `auto`. I'm glad I don't have to use C++ every day, it still feels clunky to me, even with the C++1{1,4} standard.

### Day 25 – Java

I work with Java fairly often in addition to Javascript, so there was nothing interesting about this one.

## Summary and Conclusion

For better or worse it's pretty obvious that the semantics of C continue to influence language design to this day. Curly braces for blocks, looping with `for`, `&&` and `||` for boolean operations, etc are all common in modern languages. Although studies have found these constructs to be unintuitive or extremely unintuitive [1], they are at least consistent. Even though "and" and "or" were found to be most intuitive, that would only apply for English speakers. Some languages like Ruby have moved away from this convention, if they even had it in the first place. Elixir gets a special mention for the strange choice of making `&&&` a bitwise operator, instead of the usual `&`. Although it comes down to bike-shedding for what to make your operators or keywords, I think being consistent with the majority of languages is the right way to go.

I didn’t have time to thoroughly research each language that I tried, so [Learn X in Y minutes](https://wwww.learnxinyminutes.com) and [Tutorials Point](https://www.tutorialspoint.com), along with SO obviously, were very helpful. I found most of the functional programming languages had subtle differences, so I would've liked a webpage that had information like "what you call 'foldl' in Haskell, we call 'List.fold' in F#". I know that my use case is not very common, but I think something like this list of [functional programming jargon](https://github.com/hemanth/functional-programming-jargon) translated into various FP languages would be useful for beginners. I also now consider pipeline operators a requirement for a good FP language syntax.

All in all I had a great time, and liked flexing my problem-solving muscles. The private leaderboard aspect was a fun way to compare myself to my friends, although those on the west coast had an unfair advantage. I'm not sure how much I learned about other languages, but at least I know where I'll go for my next side project. If you want to challenge me next year with a fun constraint, let me know.

[1] Stefik, Andreas, and Ed Gellenbeck. “Empirical Studies on Programming Language Stimuli.”SpringerLink, Springer US, 21 Aug. 2010, link.springer.com/article/10.1007/s11219-010-9106-7.
