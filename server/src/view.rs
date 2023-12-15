use actix_web as web;
use actix_web::web::Data;
use chrono::{
    Local,
    NaiveDateTime,
    TimeZone,
};
use maud::{
    html,
    Markup,
    PreEscaped,
    DOCTYPE,
};
use sqlx::SqlitePool;

#[web::get("/view")]
async fn view(data: Data<SqlitePool>) -> web::Result<Markup> {
    let reminders = sqlx::query_as::<_, (NaiveDateTime, String)>(
        r#"
        SELECT
            date, message
        FROM
            reminders
        WHERE
            date > datetime("now")
        ORDER BY date ASC
    "#,
    )
    .fetch_all(&**data)
    .await
    .expect("Failed to fetch reminders.");

    println!("{reminders:?}");

    let formatted_reminders: Vec<(i64, String, String)> = reminders
        .into_iter()
        .map(|(date_time, message)| {
            let local_time = Local.from_local_datetime(&date_time).unwrap();
            (
                local_time.timestamp() as i64,
                local_time.format("%Y/%m/%d %H:%M").to_string(),
                message,
            )
        })
        .collect();

    Ok(html! {
        (DOCTYPE)

        style {r#"
            body {
                font-family: sans;
                background-color: #f4f4f4;
                margin: 0;
                padding: 20px;
            }

            ul {
                list-style: none;
                padding: 0;
            }

            ul li {
                background-color: #fff;
                border-radius: 8px;
                box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
                padding: 20px;
                margin-bottom: 20px;
            }

            ul li h3 {
                margin-bottom: 10px;
                font-size: 18px;
                color: #333;
            }

            ul li p {
                color: #666;
            }
        "#}

        ul id="reminders" {
            @for reminder in formatted_reminders {
                li {
                    p class="timestamp" data-timestamp=(reminder.0) style="display: none;" {}
                    h3 { (reminder.1) }
                    p { (reminder.2) }
                }
            }
        }

        script defer {(PreEscaped(r##"
            const alertIfTime = () => {
                const reminders = Array.from(
                        document
                        .querySelectorAll("#reminders .timestamp")
                    )
                    .map(timestampItem => ({
                        content: timestampItem
                            .parentNode
                            .querySelector("p:not(.timestamp)")
                            .textContent,
                        timestamp: +timestampItem.getAttribute("data-timestamp"),
                    }));

                const currentTime = Math.floor(Date.now() / 1000);

                reminders.forEach(reminder => {
                    const differenceSeconds = currentTime - reminder.timestamp;

                    if (differenceSeconds < 1 * 60) {
                        // new Audio("/beep.mp3").play();
                        alert("Geldi! " + reminder.content);
                    }
                });
            };

            alertIfTime();

            setInterval(() => {
                location.reload();
                alertIfTime();
            }, 1 * 60 * 1000);
        "##))}
    })
}