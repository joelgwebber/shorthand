use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
//  match top_phonemes() {
  match subsequences() {
    Ok(()) => (),
    Err(e) => println!("{}", e),
  }
}

fn subsequences() -> Result<(), Error> {
  let freqs = freqs()?;
  let phonemes = phonemes()?;

  let mut seqs = HashMap::<Vec<String>, u64>::new();
  let mut insert = |seq: Vec<String>, freq: u64| {
    if seq.len() < 2 {
      return;
    }

    let mut f = freq;
    if seqs.contains_key(&seq) {
      f = f + *seqs.get(&seq).unwrap();
    }
    seqs.insert(seq, f);
  };

  for (word, freq) in freqs {
    let ophones = phonemes.get(&word);
    if ophones.is_none() {
      println!("Couldn't find '{}'", word);
      continue;
    }
    let mut seq = Vec::<String>::new();
    for phone in ophones.unwrap() {
      seq.push(phone.clone());
      insert(seq.clone(), freq);
    }
    while seq.len() > 0 {
      seq.remove(0);
      insert(seq.clone(), freq);
    }
  }

  for (seq, freq) in seqs {
    print!("{}, ", freq);
    for ph in seq {
      print!("{} ", ph);
    }
    println!();
  }

  Ok(())
}

fn top_phonemes() -> Result<(), Error> {
  let freqs = freqs()?;
  let phonemes = phonemes()?;

  let mut phfreq = HashMap::<String, u64>::new();
  for (word, freq) in freqs {
    let ophones = phonemes.get(&word);
    if ophones.is_none() {
      println!("Couldn't find '{}'", word);
      continue;
    }
    for phone in ophones.unwrap() {
      let ofr = phfreq.get(phone);
      match ofr {
        Some(cur) => phfreq.insert(phone.clone(), cur.clone() + freq),
        None => phfreq.insert(phone.clone(), freq),
      };
    }
  }

  for (phone, freq) in phfreq {
    println!("{}, {}", phone, freq);
  }

  Ok(())
}

fn phonemes() -> Result<HashMap<String, Vec<String>>, Error> {
  let filename = "cmudict.txt";
  let file = File::open(filename)?;
  let reader = BufReader::new(file);

  let mut phonemes: HashMap<String, Vec<String>> = HashMap::new();
  for line in reader.lines() {
    let line = line?;
    let mut words = line.split_whitespace();
    let word = String::from(words.next().unwrap().to_lowercase());
    if word.starts_with(";;;") {
      continue;
    }

    let mut bits = Vec::<String>::new();
    for mut part in words {
      if part.len() == 3 {
        part = part.get(0..2).unwrap();
      }
      bits.push(String::from(part));
    }
    phonemes.insert(word, bits);
  }

  Ok(phonemes)
}

fn freqs() -> Result<HashMap<String, u64>, Error> {
  let filename = "freq.txt";
  let file = File::open(filename)?;
  let reader = BufReader::new(file);

  let mut freqs: HashMap<String, u64> = HashMap::new();
  for line in reader.lines() {
    let line = line?;
    let mut words = line.split_whitespace();
    let word = String::from(words.next().unwrap().to_lowercase());
    let count = words.next().unwrap().parse::<u64>().unwrap();
    freqs.insert(word, count);
  }

  Ok(freqs)
}

fn bar() -> Result<&'static str, Error>{
  let val = foo()?;
  Ok("foo")
}

fn foo() -> Result<uint32, Error> {
  Ok(42)
}
