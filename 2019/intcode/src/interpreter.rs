use crate::executor::Executor;
use core::fmt;
use std::error::Error;
use tokio::sync::mpsc;

pub struct Interpreter {
    executor: Executor,
    output_rx: mpsc::Receiver<i64>,
    input_tx: Option<mpsc::Sender<i64>>,
}

#[allow(clippy::missing_errors_doc)]
impl Interpreter {
    pub async fn from_file(
        file: &str,
        input: Vec<i64>,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let (output_tx, output_rx) = mpsc::channel(32);
        let (input_tx, input_rx) = mpsc::channel(32);

        for i in input.clone() {
            input_tx.send(i).await?;
        }

        Ok(Self {
            executor: Executor::from_file(file, input_rx, output_tx)?,
            output_rx,
            input_tx: Some(input_tx),
        })
    }

    /// # Panics
    ///
    /// Panics if the input channel is closed but the program expected input
    pub async fn input(&mut self, val: i64) {
        self.input_tx
            .as_mut()
            .expect("Input channel doesn't exist")
            .send(val)
            .await
            .expect("Input channel closed");
    }

    pub async fn output(&mut self) -> Option<i64> {
        self.output_rx.recv().await
    }

    #[must_use]
    pub fn output_history(&self) -> &[i64] {
        self.executor.output_history()
    }

    pub async fn exec_one(&mut self) -> Result<Option<i64>, Box<dyn Error + Send + Sync>> {
        self.executor.exec_one().await
    }

    pub async fn exec(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.executor.exec().await
    }

    pub fn exec_spawn(
        mut self,
    ) -> Result<(mpsc::Sender<i64>, mpsc::Receiver<i64>), Box<dyn Error + Send + Sync>> {
        tokio::spawn(async move { self.executor.exec().await });

        Ok((self.input_tx.ok_or("Input already closed")?, self.output_rx))
    }
}

impl fmt::Debug for Interpreter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.executor)?;
        Ok(())
    }
}

impl fmt::Display for Interpreter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.executor)?;
        Ok(())
    }
}
