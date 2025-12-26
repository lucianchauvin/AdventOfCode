#[test]
fn p1() {
    const INPUT: &str = P1;

    let mut n = 0;
    for range in INPUT.split(",").map(|x| x.split("-").map(|y| y.parse::<u64>().unwrap()).collect::<Vec<u64>>()) {
        for val in range[0]..=range[1] {
            let s = val.to_string();
            if s.len().is_multiple_of(2) {
                if s.get(..(s.len()/2)) == s.get((s.len()/2)..) {
                    n += val;
                }
            }
        }
    }
    println!("{}", n);
}

fn divisors(n: usize) -> Vec<usize> {
    (1..=(n/2)).filter(|x| n.is_multiple_of(*x)).collect::<Vec<usize>>()
}

#[test]
fn p2() {
    const INPUT: &str = P1;

    let mut n = 0;
    for range in INPUT.split(",").map(|x| x.split("-").map(|y| y.parse::<usize>().unwrap()).collect::<Vec<usize>>()) {
        for val in range[0]..=range[1] {
            let s = val.to_string();
            for div in divisors(s.len()) {
                if s.chars().collect::<Vec<_>>().chunks(div).collect::<Vec<_>>().windows(2).all(|w| w[0] == w[1]) {
                    n += val;
                    break;
                }
            }
        }
    }
    println!("{}", n);
}

fn main() {
    for i in 0..100 {
        println!("{} {:?}", i, divisors(i));
    }
}

const TEST: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

const P1: &str = "5542145-5582046,243-401,884211-917063,1174-1665,767028-791710,308275-370459,285243789-285316649,3303028-3361832,793080-871112,82187-123398,7788-14096,21-34,33187450-33443224,2750031-2956556,19974-42168,37655953-37738891,1759-2640,55544-75026,9938140738-9938223673,965895186-966026269,502675-625082,11041548-11204207,1-20,3679-7591,8642243-8776142,40-88,2872703083-2872760877,532-998,211488-230593,3088932-3236371,442734-459620,8484829519-8484873271,5859767462-5859911897,9987328-10008767,656641-673714,262248430-262271846";
