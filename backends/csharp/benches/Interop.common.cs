// Automatically generated by Interoptopus.

#pragma warning disable 0105
using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Runtime.CompilerServices;
#if UNITY_2018_1_OR_NEWER
using Unity.Collections.LowLevel.Unsafe;
using Unity.Collections;
#endif
using My.Company;
using My.Company.Common;
#pragma warning restore 0105

namespace My.Company.Common
{

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Vec
    {
        public double x;
        public double z;
    }

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate byte InteropDelegate_fn_u8_rval_u8(byte x0);

    ///A pointer to an array of data someone else owns which may not be modified.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct SliceBool
    {
        ///Pointer to start of immutable data.
        #if UNITY_2018_1_OR_NEWER
        [NativeDisableUnsafePtrRestriction]
        #endif
        IntPtr data;
        ///Number of elements.
        ulong len;
    }

    public partial struct SliceBool : IEnumerable<Bool>
    {
        public SliceBool(GCHandle handle, ulong count)
        {
            this.data = handle.AddrOfPinnedObject();
            this.len = count;
        }
        public SliceBool(IntPtr handle, ulong count)
        {
            this.data = handle;
            this.len = count;
        }
        #if (NETSTANDARD2_1_OR_GREATER || NET5_0_OR_GREATER || NETCOREAPP2_1_OR_GREATER)
        public ReadOnlySpan<Bool> ReadOnlySpan
        {
            get
            {
                unsafe
                {
                    return new ReadOnlySpan<Bool>(this.data.ToPointer(), (int) this.len);
                }
            }
        }
        #endif
        #if UNITY_2018_1_OR_NEWER
        public SliceBool(NativeArray<Bool> handle)
        {
            unsafe
            {
                this.data = new IntPtr(NativeArrayUnsafeUtility.GetUnsafeReadOnlyPtr(handle));
                this.len = (ulong) handle.Length;
            }
        }
        #endif
        public Bool this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                unsafe
                {
                    var d = (Bool*) data.ToPointer();
                    return d[i];
                }
            }
        }
        public Bool[] Copied
        {
            get
            {
                var rval = new Bool[len];
                unsafe
                {
                    fixed (void* dst = rval)
                    {
                        #if __INTEROPTOPUS_NEVER
                        #elif NETCOREAPP
                        Unsafe.CopyBlock(dst, data.ToPointer(), (uint) len * (uint) sizeof(Bool));
                        #elif UNITY_2018_1_OR_NEWER
                        UnsafeUtility.MemCpy(dst, data.ToPointer(), (long) (len * (ulong) sizeof(Bool)));
                        #else
                        for (var i = 0; i < (int) len; i++) {
                            rval[i] = this[i];
                        }
                        #endif
                    }
                }
                return rval;
            }
        }
        public int Count => (int) len;
        public IEnumerator<Bool> GetEnumerator()
        {
            for (var i = 0; i < (int)len; ++i)
            {
                yield return this[i];
            }
        }
        IEnumerator IEnumerable.GetEnumerator()
        {
            return this.GetEnumerator();
        }
    }


    ///A pointer to an array of data someone else owns which may not be modified.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct SliceVec
    {
        ///Pointer to start of immutable data.
        #if UNITY_2018_1_OR_NEWER
        [NativeDisableUnsafePtrRestriction]
        #endif
        IntPtr data;
        ///Number of elements.
        ulong len;
    }

    public partial struct SliceVec : IEnumerable<Vec>
    {
        public SliceVec(GCHandle handle, ulong count)
        {
            this.data = handle.AddrOfPinnedObject();
            this.len = count;
        }
        public SliceVec(IntPtr handle, ulong count)
        {
            this.data = handle;
            this.len = count;
        }
        #if (NETSTANDARD2_1_OR_GREATER || NET5_0_OR_GREATER || NETCOREAPP2_1_OR_GREATER)
        public ReadOnlySpan<Vec> ReadOnlySpan
        {
            get
            {
                unsafe
                {
                    return new ReadOnlySpan<Vec>(this.data.ToPointer(), (int) this.len);
                }
            }
        }
        #endif
        #if UNITY_2018_1_OR_NEWER
        public SliceVec(NativeArray<Vec> handle)
        {
            unsafe
            {
                this.data = new IntPtr(NativeArrayUnsafeUtility.GetUnsafeReadOnlyPtr(handle));
                this.len = (ulong) handle.Length;
            }
        }
        #endif
        public Vec this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                unsafe
                {
                    var d = (Vec*) data.ToPointer();
                    return d[i];
                }
            }
        }
        public Vec[] Copied
        {
            get
            {
                var rval = new Vec[len];
                unsafe
                {
                    fixed (void* dst = rval)
                    {
                        #if __INTEROPTOPUS_NEVER
                        #elif NETCOREAPP
                        Unsafe.CopyBlock(dst, data.ToPointer(), (uint) len * (uint) sizeof(Vec));
                        #elif UNITY_2018_1_OR_NEWER
                        UnsafeUtility.MemCpy(dst, data.ToPointer(), (long) (len * (ulong) sizeof(Vec)));
                        #else
                        for (var i = 0; i < (int) len; i++) {
                            rval[i] = this[i];
                        }
                        #endif
                    }
                }
                return rval;
            }
        }
        public int Count => (int) len;
        public IEnumerator<Vec> GetEnumerator()
        {
            for (var i = 0; i < (int)len; ++i)
            {
                yield return this[i];
            }
        }
        IEnumerator IEnumerable.GetEnumerator()
        {
            return this.GetEnumerator();
        }
    }


    ///A pointer to an array of data someone else owns which may not be modified.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Sliceu32
    {
        ///Pointer to start of immutable data.
        #if UNITY_2018_1_OR_NEWER
        [NativeDisableUnsafePtrRestriction]
        #endif
        IntPtr data;
        ///Number of elements.
        ulong len;
    }

    public partial struct Sliceu32 : IEnumerable<uint>
    {
        public Sliceu32(GCHandle handle, ulong count)
        {
            this.data = handle.AddrOfPinnedObject();
            this.len = count;
        }
        public Sliceu32(IntPtr handle, ulong count)
        {
            this.data = handle;
            this.len = count;
        }
        #if (NETSTANDARD2_1_OR_GREATER || NET5_0_OR_GREATER || NETCOREAPP2_1_OR_GREATER)
        public ReadOnlySpan<uint> ReadOnlySpan
        {
            get
            {
                unsafe
                {
                    return new ReadOnlySpan<uint>(this.data.ToPointer(), (int) this.len);
                }
            }
        }
        #endif
        #if UNITY_2018_1_OR_NEWER
        public Sliceu32(NativeArray<uint> handle)
        {
            unsafe
            {
                this.data = new IntPtr(NativeArrayUnsafeUtility.GetUnsafeReadOnlyPtr(handle));
                this.len = (ulong) handle.Length;
            }
        }
        #endif
        public uint this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                unsafe
                {
                    var d = (uint*) data.ToPointer();
                    return d[i];
                }
            }
        }
        public uint[] Copied
        {
            get
            {
                var rval = new uint[len];
                unsafe
                {
                    fixed (void* dst = rval)
                    {
                        #if __INTEROPTOPUS_NEVER
                        #elif NETCOREAPP
                        Unsafe.CopyBlock(dst, data.ToPointer(), (uint) len * (uint) sizeof(uint));
                        #elif UNITY_2018_1_OR_NEWER
                        UnsafeUtility.MemCpy(dst, data.ToPointer(), (long) (len * (ulong) sizeof(uint)));
                        #else
                        for (var i = 0; i < (int) len; i++) {
                            rval[i] = this[i];
                        }
                        #endif
                    }
                }
                return rval;
            }
        }
        public int Count => (int) len;
        public IEnumerator<uint> GetEnumerator()
        {
            for (var i = 0; i < (int)len; ++i)
            {
                yield return this[i];
            }
        }
        IEnumerator IEnumerable.GetEnumerator()
        {
            return this.GetEnumerator();
        }
    }


    ///A pointer to an array of data someone else owns which may not be modified.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Sliceu8
    {
        ///Pointer to start of immutable data.
        #if UNITY_2018_1_OR_NEWER
        [NativeDisableUnsafePtrRestriction]
        #endif
        IntPtr data;
        ///Number of elements.
        ulong len;
    }

    public partial struct Sliceu8 : IEnumerable<byte>
    {
        public Sliceu8(GCHandle handle, ulong count)
        {
            this.data = handle.AddrOfPinnedObject();
            this.len = count;
        }
        public Sliceu8(IntPtr handle, ulong count)
        {
            this.data = handle;
            this.len = count;
        }
        #if (NETSTANDARD2_1_OR_GREATER || NET5_0_OR_GREATER || NETCOREAPP2_1_OR_GREATER)
        public ReadOnlySpan<byte> ReadOnlySpan
        {
            get
            {
                unsafe
                {
                    return new ReadOnlySpan<byte>(this.data.ToPointer(), (int) this.len);
                }
            }
        }
        #endif
        #if UNITY_2018_1_OR_NEWER
        public Sliceu8(NativeArray<byte> handle)
        {
            unsafe
            {
                this.data = new IntPtr(NativeArrayUnsafeUtility.GetUnsafeReadOnlyPtr(handle));
                this.len = (ulong) handle.Length;
            }
        }
        #endif
        public byte this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                unsafe
                {
                    var d = (byte*) data.ToPointer();
                    return d[i];
                }
            }
        }
        public byte[] Copied
        {
            get
            {
                var rval = new byte[len];
                unsafe
                {
                    fixed (void* dst = rval)
                    {
                        #if __INTEROPTOPUS_NEVER
                        #elif NETCOREAPP
                        Unsafe.CopyBlock(dst, data.ToPointer(), (uint) len * (uint) sizeof(byte));
                        #elif UNITY_2018_1_OR_NEWER
                        UnsafeUtility.MemCpy(dst, data.ToPointer(), (long) (len * (ulong) sizeof(byte)));
                        #else
                        for (var i = 0; i < (int) len; i++) {
                            rval[i] = this[i];
                        }
                        #endif
                    }
                }
                return rval;
            }
        }
        public int Count => (int) len;
        public IEnumerator<byte> GetEnumerator()
        {
            for (var i = 0; i < (int)len; ++i)
            {
                yield return this[i];
            }
        }
        IEnumerator IEnumerable.GetEnumerator()
        {
            return this.GetEnumerator();
        }
    }


    ///A pointer to an array of data someone else owns which may be modified.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct SliceMut*const i8
    {
        ///Pointer to start of mutable data.
        #if UNITY_2018_1_OR_NEWER
        [NativeDisableUnsafePtrRestriction]
        #endif
        IntPtr data;
        ///Number of elements.
        ulong len;
    }

    public partial struct SliceMut*const i8 : IEnumerable<IntPtr>
    {
        public SliceMut*const i8(GCHandle handle, ulong count)
        {
            this.data = handle.AddrOfPinnedObject();
            this.len = count;
        }
        public SliceMut*const i8(IntPtr handle, ulong count)
        {
            this.data = handle;
            this.len = count;
        }
        #if (NETSTANDARD2_1_OR_GREATER || NET5_0_OR_GREATER || NETCOREAPP2_1_OR_GREATER)
        public ReadOnlySpan<IntPtr> ReadOnlySpan
        {
            get
            {
                unsafe
                {
                    return new ReadOnlySpan<IntPtr>(this.data.ToPointer(), (int) this.len);
                }
            }
        }
        #endif
        #if UNITY_2018_1_OR_NEWER
        public SliceMut*const i8(NativeArray<IntPtr> handle)
        {
            unsafe
            {
                this.data = new IntPtr(NativeArrayUnsafeUtility.GetUnsafeReadOnlyPtr(handle));
                this.len = (ulong) handle.Length;
            }
        }
        #endif
        #if (NETSTANDARD2_1_OR_GREATER || NET5_0_OR_GREATER || NETCOREAPP2_1_OR_GREATER)
        public Span<IntPtr> Span
        {
            get
            {
                unsafe
                {
                    return new Span<IntPtr>(this.data.ToPointer(), (int) this.len);
                }
            }
        }
        #endif
        public IntPtr this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                unsafe
                {
                    var d = (IntPtr*) data.ToPointer();
                    return d[i];
                }
            }
            set
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                unsafe
                {
                    var d = (IntPtr*) data.ToPointer();
                    d[i] = value;
                }
            }
        }
        public IntPtr[] Copied
        {
            get
            {
                var rval = new IntPtr[len];
                unsafe
                {
                    fixed (void* dst = rval)
                    {
                        #if __FALSE
                        #elif NETCOREAPP
                        Unsafe.CopyBlock(dst, data.ToPointer(), (uint) len * (uint) sizeof(IntPtr));
                        #elif UNITY_2018_1_OR_NEWER
                        UnsafeUtility.MemCpy(dst, data.ToPointer(), (long) (len * (ulong) sizeof(IntPtr)));
                        #else
                        for (var i = 0; i < (int) len; i++) {
                            rval[i] = this[i];
                        }
                        #endif
                    }
                }
                return rval;
            }
        }
        public int Count => (int) len;
        public IEnumerator<IntPtr> GetEnumerator()
        {
            for (var i = 0; i < (int)len; ++i)
            {
                yield return this[i];
            }
        }
        IEnumerator IEnumerable.GetEnumerator()
        {
            return this.GetEnumerator();
        }
    }


    ///A pointer to an array of data someone else owns which may be modified.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct SliceMutVec
    {
        ///Pointer to start of mutable data.
        #if UNITY_2018_1_OR_NEWER
        [NativeDisableUnsafePtrRestriction]
        #endif
        IntPtr data;
        ///Number of elements.
        ulong len;
    }

    public partial struct SliceMutVec : IEnumerable<Vec>
    {
        public SliceMutVec(GCHandle handle, ulong count)
        {
            this.data = handle.AddrOfPinnedObject();
            this.len = count;
        }
        public SliceMutVec(IntPtr handle, ulong count)
        {
            this.data = handle;
            this.len = count;
        }
        #if (NETSTANDARD2_1_OR_GREATER || NET5_0_OR_GREATER || NETCOREAPP2_1_OR_GREATER)
        public ReadOnlySpan<Vec> ReadOnlySpan
        {
            get
            {
                unsafe
                {
                    return new ReadOnlySpan<Vec>(this.data.ToPointer(), (int) this.len);
                }
            }
        }
        #endif
        #if UNITY_2018_1_OR_NEWER
        public SliceMutVec(NativeArray<Vec> handle)
        {
            unsafe
            {
                this.data = new IntPtr(NativeArrayUnsafeUtility.GetUnsafeReadOnlyPtr(handle));
                this.len = (ulong) handle.Length;
            }
        }
        #endif
        #if (NETSTANDARD2_1_OR_GREATER || NET5_0_OR_GREATER || NETCOREAPP2_1_OR_GREATER)
        public Span<Vec> Span
        {
            get
            {
                unsafe
                {
                    return new Span<Vec>(this.data.ToPointer(), (int) this.len);
                }
            }
        }
        #endif
        public Vec this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                unsafe
                {
                    var d = (Vec*) data.ToPointer();
                    return d[i];
                }
            }
            set
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                unsafe
                {
                    var d = (Vec*) data.ToPointer();
                    d[i] = value;
                }
            }
        }
        public Vec[] Copied
        {
            get
            {
                var rval = new Vec[len];
                unsafe
                {
                    fixed (void* dst = rval)
                    {
                        #if __FALSE
                        #elif NETCOREAPP
                        Unsafe.CopyBlock(dst, data.ToPointer(), (uint) len * (uint) sizeof(Vec));
                        #elif UNITY_2018_1_OR_NEWER
                        UnsafeUtility.MemCpy(dst, data.ToPointer(), (long) (len * (ulong) sizeof(Vec)));
                        #else
                        for (var i = 0; i < (int) len; i++) {
                            rval[i] = this[i];
                        }
                        #endif
                    }
                }
                return rval;
            }
        }
        public int Count => (int) len;
        public IEnumerator<Vec> GetEnumerator()
        {
            for (var i = 0; i < (int)len; ++i)
            {
                yield return this[i];
            }
        }
        IEnumerator IEnumerable.GetEnumerator()
        {
            return this.GetEnumerator();
        }
    }


    ///A pointer to an array of data someone else owns which may be modified.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct SliceMutu32
    {
        ///Pointer to start of mutable data.
        #if UNITY_2018_1_OR_NEWER
        [NativeDisableUnsafePtrRestriction]
        #endif
        IntPtr data;
        ///Number of elements.
        ulong len;
    }

    public partial struct SliceMutu32 : IEnumerable<uint>
    {
        public SliceMutu32(GCHandle handle, ulong count)
        {
            this.data = handle.AddrOfPinnedObject();
            this.len = count;
        }
        public SliceMutu32(IntPtr handle, ulong count)
        {
            this.data = handle;
            this.len = count;
        }
        #if (NETSTANDARD2_1_OR_GREATER || NET5_0_OR_GREATER || NETCOREAPP2_1_OR_GREATER)
        public ReadOnlySpan<uint> ReadOnlySpan
        {
            get
            {
                unsafe
                {
                    return new ReadOnlySpan<uint>(this.data.ToPointer(), (int) this.len);
                }
            }
        }
        #endif
        #if UNITY_2018_1_OR_NEWER
        public SliceMutu32(NativeArray<uint> handle)
        {
            unsafe
            {
                this.data = new IntPtr(NativeArrayUnsafeUtility.GetUnsafeReadOnlyPtr(handle));
                this.len = (ulong) handle.Length;
            }
        }
        #endif
        #if (NETSTANDARD2_1_OR_GREATER || NET5_0_OR_GREATER || NETCOREAPP2_1_OR_GREATER)
        public Span<uint> Span
        {
            get
            {
                unsafe
                {
                    return new Span<uint>(this.data.ToPointer(), (int) this.len);
                }
            }
        }
        #endif
        public uint this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                unsafe
                {
                    var d = (uint*) data.ToPointer();
                    return d[i];
                }
            }
            set
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                unsafe
                {
                    var d = (uint*) data.ToPointer();
                    d[i] = value;
                }
            }
        }
        public uint[] Copied
        {
            get
            {
                var rval = new uint[len];
                unsafe
                {
                    fixed (void* dst = rval)
                    {
                        #if __FALSE
                        #elif NETCOREAPP
                        Unsafe.CopyBlock(dst, data.ToPointer(), (uint) len * (uint) sizeof(uint));
                        #elif UNITY_2018_1_OR_NEWER
                        UnsafeUtility.MemCpy(dst, data.ToPointer(), (long) (len * (ulong) sizeof(uint)));
                        #else
                        for (var i = 0; i < (int) len; i++) {
                            rval[i] = this[i];
                        }
                        #endif
                    }
                }
                return rval;
            }
        }
        public int Count => (int) len;
        public IEnumerator<uint> GetEnumerator()
        {
            for (var i = 0; i < (int)len; ++i)
            {
                yield return this[i];
            }
        }
        IEnumerator IEnumerable.GetEnumerator()
        {
            return this.GetEnumerator();
        }
    }


    ///A pointer to an array of data someone else owns which may be modified.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct SliceMutu8
    {
        ///Pointer to start of mutable data.
        #if UNITY_2018_1_OR_NEWER
        [NativeDisableUnsafePtrRestriction]
        #endif
        IntPtr data;
        ///Number of elements.
        ulong len;
    }

    public partial struct SliceMutu8 : IEnumerable<byte>
    {
        public SliceMutu8(GCHandle handle, ulong count)
        {
            this.data = handle.AddrOfPinnedObject();
            this.len = count;
        }
        public SliceMutu8(IntPtr handle, ulong count)
        {
            this.data = handle;
            this.len = count;
        }
        #if (NETSTANDARD2_1_OR_GREATER || NET5_0_OR_GREATER || NETCOREAPP2_1_OR_GREATER)
        public ReadOnlySpan<byte> ReadOnlySpan
        {
            get
            {
                unsafe
                {
                    return new ReadOnlySpan<byte>(this.data.ToPointer(), (int) this.len);
                }
            }
        }
        #endif
        #if UNITY_2018_1_OR_NEWER
        public SliceMutu8(NativeArray<byte> handle)
        {
            unsafe
            {
                this.data = new IntPtr(NativeArrayUnsafeUtility.GetUnsafeReadOnlyPtr(handle));
                this.len = (ulong) handle.Length;
            }
        }
        #endif
        #if (NETSTANDARD2_1_OR_GREATER || NET5_0_OR_GREATER || NETCOREAPP2_1_OR_GREATER)
        public Span<byte> Span
        {
            get
            {
                unsafe
                {
                    return new Span<byte>(this.data.ToPointer(), (int) this.len);
                }
            }
        }
        #endif
        public byte this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                unsafe
                {
                    var d = (byte*) data.ToPointer();
                    return d[i];
                }
            }
            set
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                unsafe
                {
                    var d = (byte*) data.ToPointer();
                    d[i] = value;
                }
            }
        }
        public byte[] Copied
        {
            get
            {
                var rval = new byte[len];
                unsafe
                {
                    fixed (void* dst = rval)
                    {
                        #if __FALSE
                        #elif NETCOREAPP
                        Unsafe.CopyBlock(dst, data.ToPointer(), (uint) len * (uint) sizeof(byte));
                        #elif UNITY_2018_1_OR_NEWER
                        UnsafeUtility.MemCpy(dst, data.ToPointer(), (long) (len * (ulong) sizeof(byte)));
                        #else
                        for (var i = 0; i < (int) len; i++) {
                            rval[i] = this[i];
                        }
                        #endif
                    }
                }
                return rval;
            }
        }
        public int Count => (int) len;
        public IEnumerator<byte> GetEnumerator()
        {
            for (var i = 0; i < (int)len; ++i)
            {
                yield return this[i];
            }
        }
        IEnumerator IEnumerable.GetEnumerator()
        {
            return this.GetEnumerator();
        }
    }


    ///Option type containing boolean flag and maybe valid data.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct OptionVec
    {
        ///Element that is maybe valid.
        Vec t;
        ///Byte where `1` means element `t` is valid.
        byte is_some;
    }

    public partial struct OptionVec
    {
        public static OptionVec FromNullable(Vec? nullable)
        {
            var result = new OptionVec();
            if (nullable.HasValue)
            {
                result.is_some = 1;
                result.t = nullable.Value;
            }

            return result;
        }

        public Vec? ToNullable()
        {
            return this.is_some == 1 ? this.t : (Vec?)null;
        }
    }


    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Bool
    {
        byte value;
    }

    public partial struct Bool
    {
        public static readonly Bool True = new Bool { value =  1 };
        public static readonly Bool False = new Bool { value =  0 };
        public Bool(bool b)
        {
            value = (byte) (b ? 1 : 0);
        }
        public bool Is => value == 1;
    }


    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate byte CallbackFFISlice(Sliceu8 slice);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate Vec3f32 CallbackHugeVecSlice(SliceVec3f32 slice);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate void CallbackSliceMut(SliceMutu8 slice);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate byte CallbackU8(byte value);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate uint MyCallback(uint value);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate void MyCallbackContextual(IntPtr context, uint value);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate uint MyCallbackNamespaced(uint value);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate void MyCallbackVoid(IntPtr ptr);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate void SumDelegate1();

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate int SumDelegate2(int x, int y);

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate FFIError SumDelegateReturn(int x, int y);



    public class InteropException<T> : Exception
    {
        public T Error { get; private set; }

        public InteropException(T error): base($"Something went wrong: {error}")
        {
            Error = error;
        }
    }

}
