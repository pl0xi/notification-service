use criterion::{criterion_group, criterion_main, Criterion};
use notification_service::services::document::create_pdf;

#[allow(unused_variables)]
fn benchmarks(c: &mut Criterion) {
    let mut pdf_creation_group = c.benchmark_group("pdf_creation");
    let pdf_content = r#"<!DOCTYPE html>
        <html>
            <body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
                <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
                    <h1 style="color: #2c3e50;">Thank You for Your Order!</h1>
                    
                    <p>Dear {{customer.first_name}} {{customer.last_name}},</p>
                    
                    <p style="font-style: italic;">note: This is not an order confirmation</p>
                </div>

                <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
                    {{> signature}}
                </div>
            </body>
        </html>"#;

    pdf_creation_group.bench_with_input("pdf_creation", &pdf_content, |b, _| b.iter(|| create_pdf(&pdf_content, "Invoice")));
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
