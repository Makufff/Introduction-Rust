trait QuestNotifier {
    fn notify(&self , message : &str);
}

struct Pigeon ;

impl QuestNotifier for Pigeon {
    fn notify(&self , message : &str) {
        println!("Pigeon Notifier : {}", message);
    }
}

struct MailCarrier ;

impl QuestNotifier for MailCarrier {
    fn notify(&self , message : &str) {
        println!("Mail Carrier Notifier : {}", message);
    }
}

struct QuestManager ;

impl QuestManager {

    fn complete_quest<T: QuestNotifier>(&self , notifier : T) {
        notifier.notify("Quest Completed");
    }
}

fn main() {

    let pigeon = Pigeon;
    let mail_carrier = MailCarrier;

    let quest_manager = QuestManager;

    quest_manager.complete_quest(pigeon);
    quest_manager.complete_quest(mail_carrier);
}
