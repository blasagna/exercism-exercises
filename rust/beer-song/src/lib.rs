pub fn verse(n: i32) -> String {
  let mut verse = String::from("");

  let amount = n.to_string();

  let n_str = if n > 0 {
    &amount
  } else {
    "No more"
  };
  verse.push_str(n_str);

  let n_sing_plural = if n != 1 {
    "bottles"
  } else {
    "bottle"
  };
  verse.push_str(" ");
  verse.push_str(n_sing_plural);
  verse.push_str(" of beer on the wall, ");

  verse.push_str(&(n_str.to_lowercase()));
  verse.push_str(" ");
  verse.push_str(n_sing_plural);
  verse.push_str(" of beer.\n");

  let m = n - 1;
  let decr_amount = m.to_string();
  let m_str = if m > 0 {
    &decr_amount
  } else {
    "no more"
  };

  let take_down = if m > 0 {
    "one"
  } else {
    "it"
  };

  if m > -1 {
    verse.push_str("Take ");
    verse.push_str(take_down);
    verse.push_str(" down and pass it around, ");

    let m_sing_plural = if m != 1 {
      "bottles"
    } else {
      "bottle"
    };

    verse.push_str(m_str);
    verse.push_str(" ");
    verse.push_str(m_sing_plural);
    verse.push_str(" of beer on the wall.\n");
  } else {
    verse.push_str("Go to the store and buy some more, 99 bottles of beer on the wall.\n");
  }

  verse
}

pub fn sing(start: i32, end: i32) -> String {
  let mut song = String::from("");

  for i in (end..(start+1)).rev() {
    song.push_str(&verse(i));
    if i != end {
      song.push_str("\n");
    }
  }

  song
}
