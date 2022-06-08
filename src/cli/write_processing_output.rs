use tokio::io::AsyncWrite;
use crate::errors::CliError;
use crate::models::MoneyCents;
use crate::processor::ProcessingOutput;
use crate::utils::MONEY_UNIT_SUBDIVISIONS;

fn fmt_money_cents(money: MoneyCents) -> String {
    (money / MONEY_UNIT_SUBDIVISIONS as i64).to_string()
}

pub async fn write_processing_output(output: ProcessingOutput, to: impl AsyncWrite + Unpin) -> Result<(), CliError> {
    let mut wri = csv_async::AsyncWriter::from_writer(
        to,
    );

    wri.write_record(&["client", "available", "held", "total", "locked"])
        .await?;

    for client in output.clients {
        wri.write_record(&[
            format!("{}", client.get_id()),
            fmt_money_cents(client.get_available()),
            fmt_money_cents(client.get_held()),
            fmt_money_cents(client.get_total()),
            client.is_locked().to_string(),
        ])
            .await?;
    }

    Ok(())
}