using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace day21_csharp
{
    using Transform = Tuple<char[][,], char[,]>;

    internal class Program
    {
        static void Main()
        {
            string[] lines = File.ReadAllLines("input");
            Transform[] transforms = lines.Select(GetTransforms).ToArray();
            char[,] pattern = {{'.', '#', '.'}, {'.', '.', '#'}, {'#', '#', '#'}};
            const int reps = 18;
            for (var i = 0; i < reps; i++)
            {
               pattern = Expand(transforms, pattern);
            }
            CountOn(pattern);
        }

        static void CountOn(char[,] pattern)
        {
            int n = pattern.GetLength(0);
            var c = 0;
            for (var i = 0; i < n; i++)
            {
                for (var j = 0; j < n; j++)
                {
                    if (pattern[i, j] == '#')
                    {
                        c++;
                    }
                }
            }
            Console.WriteLine(c);
        }

        static void PrintPattern(char[,] pattern)
        {
            int n = pattern.GetLength(0);
            for (var i = 0; i < n; i++)
            {
                for (var j = 0; j < n; j++)
                {
                    Console.Write(pattern[i, j]);
                }
                Console.Write("\n");
            }
        }

        static char[,] Expand(Transform[] transforms, char[,] pattern)
        {
            int n = pattern.GetLength(0);
            int chunkSize = n % 2 == 0 ? 2 : 3;
            int newChunkSize = chunkSize + 1;
            int m = n / chunkSize * newChunkSize;
            var newPattern = new char[m,m];
            for (int i = 0, x = 0; i < n; i += chunkSize, x += newChunkSize)
            {
                for (int j = 0, y = 0; j < n; j += chunkSize, y += newChunkSize)
                {
                    char[,] block = ExtractBlock(pattern, i, j, chunkSize);
                    Transform matchingTransform = transforms.First(t => Match(t, block));
                    SetBlock(newPattern, matchingTransform.Item2, x, y, newChunkSize);
                }
            }

            return newPattern;
        }

        static char[,] ExtractBlock(char[,] pattern, int x, int y, int size)
        {
            var block = new char[size, size];
            for (var i = 0; i < size; i++)
            {
                for (var j = 0; j < size; j++)
                {
                    block[i, j] = pattern[x + i, y + j];
                }
            }

            return block;
        }

        static void SetBlock(char[,] pattern, char[,] block, int x, int y, int size)
        {
            for (var i = 0; i < size; i++)
            {
                for (var j = 0; j < size; j++)
                {
                    pattern[x + i, y + j] = block[i, j];
                }
            }
        }

        static Transform GetTransforms(string line)
        {
            string[] parts = line.Split(" => ");
            char[][] inputRows = parts[0].Split("/").Select(row => row.ToCharArray()).ToArray();
            char[,] input = JaggedToRectangular(inputRows);

            char[][] outputRows = parts[1].Split("/").Select(row => row.ToCharArray()).ToArray();
            char[,] output = JaggedToRectangular(outputRows);

            char[,] flipX = Flip(input, true);
            char[,] flipY = Flip(input, false);
            char[][,] inputs =
            {
                input,
                Rotate(input, 1),
                Rotate(input, 2),
                Rotate(input, 3),
                flipX,
                Rotate(flipX, 1),
                Rotate(flipX, 2),
                Rotate(flipX, 3),
                flipY,
                Rotate(flipY, 1),
                Rotate(flipY, 2),
                Rotate(flipY, 3),
            };

            return new Tuple<char[][,], char[,]>(inputs, output);
        }

        static char[,] JaggedToRectangular(IReadOnlyList<char[]> array)
        {
            var rect = new char[array.Count, array[0].Length];
            for (var i = 0; i < array.Count; i++)
            {
                for (var j = 0; j < array[0].Length; j++)
                {
                    rect[i, j] = array[i][j];
                }
            }

            return rect;
        }

        static bool Match(Transform t, char[,] pattern)
        {
            return t.Item1[0].GetLength(0) == pattern.GetLength(0) && t.Item1.Any(input => MatrixEqual(input, pattern));
        }

        static char[,] Rotate(char[,] t, int times)
        {
            var newT = new char[t.GetLength(0), t.GetLength(1)];
            int n = t.GetLength(0);
            for (var i = 0; i < n; i++)
            {
                for (var j = 0; j < n; j++)
                {
                    switch (times)
                    {
                        case 1:
                            newT[j, n - i - 1] = t[i, j];
                            break;
                        case 2:
                            newT[n - i - 1, n - j - 1] = t[i, j];
                            break;
                        default:
                            newT[n - j - 1, i] = t[i, j];
                            break;
                    }
                }
            }

            return newT;
        }

        static char[,] Flip(char[,] t, bool flipX)
        {
            int n = t.GetLength(0);
            var newT = new char[n, t.GetLength(1)];
            for (var i = 0; i < n; i++)
            {
                for (var j = 0; j < n; j++)
                {
                    if (flipX)
                    {
                        newT[n - i - 1, j] = t[i, j];
                    }
                    else
                    {
                        newT[i, n - j - 1] = t[i, j];
                    }
                }
            }

            return newT;
        }

        static bool MatrixEqual(char[,] a, char[,] b)
        {
            for (var i = 0; i < a.GetLength(0); i++)
            {
                for (var j = 0; j < a.GetLength(1); j++)
                {
                    if (a[i,j] != b[i,j])
                    {
                        return false;
                    }
                }
            }

            return true;
        }
    }
}
