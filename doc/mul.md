% What #[derive(Mul)] generates

# Tuple structs

When deriving for a tuple struct with a single field (i.e. a newtype) like this:

```
#[derive(From)]
struct MyInt(i32)
```

Code like this will be generated:

```
impl<T> ::std::ops::Mul<T> for MyInt
    where T: ::std::ops::Mul<i32, Output = i32>
{
    type Output = MyInt;
    fn mul(self, rhs: T) -> MyInt {
        MyInt(rhs.mul(self.0))
    }
}
```

The behaviour is slightly different for multiple fields, since the right hand
side of the multiplication now needs the `Copy` trait.
For instance when deriving for a tuple struct with two fields like this:

```
#[derive(Mul)]
struct MyInts(i32, i32)
```

Code like this will be generated:

```
impl<T> ::std::ops::Mul<T> for MyInts
    where T: ::std::ops::Mul<i32, Output = i32> + ::std::marker::Copy
{
    type Output = MyInts;
    fn mul(self, rhs: T) -> MyInts {
        MyInts(rhs.mul(self.0), rhs.mul(self.1))
    }
}
```

The behaviour is similar with more or less fields.



# Regular structs

When deriving `Mul` for a regular struct with a single field like this:

```
#[derive(Mul)]
struct Point1D {
    x: i32,
}
```

Code like this will be generated:

```
impl<T> ::std::ops::Mul<T> for Point1D
    where T: ::std::ops::Mul<i32, Output = i32>
{
    type Output = Point1D;
    fn mul(self, rhs: T) -> Point1D {
        Point1D { x: rhs.mul(self.x) }
    }
}
```

The behaviour is again slightly different when deriving for a struct with multiple
fields, because it still needs the `Copy` as well.
For instance when deriving for a tuple struct with two fields like this:

```
#[derive(Mul)]
struct Point2D {
    x: i32,
    y: i32,
}
```

Code like this will be generated:

```
impl<T> ::std::ops::Mul<T> for Point2D
    where T: ::std::ops::Mul<i32, Output = i32> + ::std::marker::Copy
{
    type Output = Point2D;
    fn mul(self, rhs: T) -> Point2D {
        Point2D {
            x: rhs.mul(self.x),
            y: rhs.mul(self.y),
        }
    }
}
```


# Enums

Deriving `Mul` for enums is not (yet) supported.
Although it shouldn't be impossible no effort has been put into this yet.
