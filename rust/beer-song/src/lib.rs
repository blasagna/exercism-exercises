pub fn verse(n: i32) -> String {
  let amount = n.to_string();
  let n_str = if n > 0 {
    &amount
  } else {
    "No more"
  };

  let n_sing_plural = if n != 1 {
    "bottles"
  } else {
    "bottle"
  };

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

  let m_sing_plural = if m != 1 {
    "bottles"
  } else {
    "bottle"
  };

  let first = format!("{} {} of beer on the wall, {} {} of beer.", n_str, n_sing_plural, n_str.to_lowercase(), n_sing_plural);
  let second = if m >= 0 {
    format!("Take {} down and pass it around, {} {} of beer on the wall.", take_down, m_str, m_sing_plural)
  } else {
    "Go to the store and buy some more, 99 bottles of beer on the wall.".to_string()
  };

  format!("{}\n{}\n", first, second)
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
