// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

fn main() {
    for verse_number in 1..=12 {
        let verse_number_string = match verse_number {
            1 => "first",
            2 => "second",
            3 => "third",
            4 => "fourth",
            5 => "fifth",
            6 => "sixth",
            7 => "seventh",
            8 => "eighth",
            9 => "ninth",
            10 => "tenth",
            11 => "eleventh",
            12 => "twelfth",
            _ => panic!("There are only 12 verses!")
        };

        println!("On the {verse_number_string} day of Christmas,");
        println!("my true love gave to me");


        if verse_number >= 12 {
            println!("Twelve drummers drumming,");
        }
        if verse_number >= 11 {
            println!("Eleven pipers piping,");
        }
        if verse_number >= 10 {
            println!("Ten lords a-leaping,");
        }
        if verse_number >= 9 {
            println!("Nine ladies dancing,");
        }
        if verse_number >= 8 {
            println!("Eight maids a-milking,");
        }
        if verse_number >= 7 {
            println!("Seven swans a-swimming,");
        }
        if verse_number >= 6 {
            println!("Six geese a-laying,");
        }
        if verse_number >= 5 {
            println!("Five golden rings,");
        }
        if verse_number >= 4 {
            println!("Four calling birds,");
        }
        if verse_number >= 3 {
            println!("Three French hens,");
        }
        if verse_number >= 2 {
            println!("Two turtle doves,");
        }

        if verse_number == 1 {
            println!("A partridge in a pear tree.");
        } else if verse_number == 12 {
            println!("And a partridge in a pear tree!");
        } else {
            println!("And a partridge in a pear tree.");
        }

        println!("");
    }
}

/*
On the first day of Christmas,
my true love gave to me
A partridge in a pear tree.

On the second day of Christmas,
my true love gave to me
Two turtle doves,
And a partridge in a pear tree.

On the third day of Christmas,
my true love gave to me
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the fourth day of Christmas,
my true love gave to me
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the fifth day of Christmas,
my true love gave to me
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the sixth day of Christmas,
my true love gave to me
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the seventh day of Christmas,
my true love gave to me
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the eighth day of Christmas,
my true love gave to me
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the ninth day of Christmas,
my true love gave to me
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the tenth day of Christmas,
my true love gave to me
Ten lords a-leaping,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the eleventh day of Christmas,
my true love gave to me
Eleven pipers piping,
Ten lords a-leaping,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.

On the twelfth day of Christmas,
my true love gave to me
Twelve drummers drumming,
Eleven pipers piping,
Ten lords a-leaping,
Nine ladies dancing,
Eight maids a-milking,
Seven swans a-swimming,
Six geese a-laying,
Five golden rings,
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree!
 */