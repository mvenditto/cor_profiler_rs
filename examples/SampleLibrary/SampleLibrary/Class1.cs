using System;

namespace SampleLibrary
{
    public class Class1
    {
        class Class1Child { }

        private class Class1Child2 { }

        private int _value;

        public Class1(int value)
        {
            _value = value;
        }

        public Class1(): this(42)
        {

        }

        public int Add(int a, int b)
        {
            return a + b;
        }

        public object ReturnSelf()
        {
            return this;
        }

        public string ReturnString()
        {
            return "string";
        }

        public int[] ReturnArray()
        {
            return new[] { 42, 42, 42 };
        }

        public Class1[,] Return2DClassArray()
        {
            return (Class1[,]) new object[4, 2];
        }

        public Class1[,,] Return3DClassArray()
        {
            return (Class1[,,]) new object[4, 2, 3];
        }

        public int[,] Return2DIntArray()
        {
            return new int[4, 2];
        }

        public int[,,] Return3DIntArray()
        {
            return new int[4, 2, 3];
        }

        public int[][] ReturnJaggedIntArray()
        {
            return new int[3][];
        }

        public Class1[][] ReturnJaggedClassArray()
        {
            return new Class1[][]
            {
                new Class1[] { },
                new Class1[] { new Class1() }
            };
        }
    }
}
