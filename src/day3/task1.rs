use crate::util::AoCError;

pub fn calculate(input: &Vec<String>) -> Result<PowerConsumption, AoCError> {
    let len = input.first().map(|s| s.len()).ok_or_else(|| AoCError::new("Input vec must not be empty".to_owned()))?;
    let nums = input.iter()
        .filter(|s| !s.is_empty())
        .map(|s| usize::from_str_radix(s, 2).map_err(|e| AoCError::new(format!("Failed to parse String. {}", e.to_string()))))
        .collect::<Result<Vec<usize>, AoCError>>()?;
    return Ok(calculate_binary(&nums, len));
}

fn calculate_binary(input: &Vec<usize>, len: usize) -> PowerConsumption {
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..len {
        let mut ones = 0;
        for j in input {
            let r = j >> i & 0b1;
            ones += r;
        }
        if ones > (input.len() / 2) {
            gamma |= 0b1 << i;
            epsilon |= 0b0 << i;
        } else {
            gamma |= 0b0 << i;
            epsilon |= 0b1 << i;
        };
    }
    PowerConsumption { gamma, epsilon }
}

pub struct PowerConsumption {
    pub gamma: usize,
    pub epsilon: usize,
}

impl PowerConsumption {
    pub fn score(&self) -> usize {
        self.gamma * self.epsilon
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return() {
        // given
        let input = vec!["00100".to_owned(), "11110".to_owned(), "10110".to_owned(), "10111".to_owned(), "10101".to_owned(), "01111".to_owned(), "00111".to_owned(), "11100".to_owned(), "10000".to_owned(), "11001".to_owned(), "00010".to_owned(), "01010".to_owned()];

        // when
        let result = calculate(&input).unwrap();

        // then
        assert_eq!(result.gamma, 22);
        assert_eq!(result.epsilon, 9);
        assert_eq!(result.score(), 198);
    }
}
