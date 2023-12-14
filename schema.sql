use dev;

CREATE TABLE `device_groups` (
                                 `device_group_id` int unsigned NOT NULL AUTO_INCREMENT,
                                 `serial_number` varchar(100) NOT NULL,
                                 `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                 PRIMARY KEY (`device_group_id`),
                                 UNIQUE KEY `DEVICE_UQ` (`serial_number`),
                                 KEY `DATE_IDX` (`serial_number`,`created_at`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE TABLE `devices` (
                           `device_id` int unsigned NOT NULL AUTO_INCREMENT,
                           `device_group_id` int unsigned NOT NULL,
                           `serial_number` varchar(100) NOT NULL,
                           `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
                           PRIMARY KEY (`device_id`),
                           UNIQUE KEY `DEVICE_UQ` (`serial_number`),
                           KEY `group_pk_idx` (`device_group_id`),
                           KEY `DATE_IDX` (`serial_number`,`created_at`),
                           CONSTRAINT `group_pk` FOREIGN KEY (`device_group_id`) REFERENCES `device_groups` (`device_group_id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE TABLE `temperatures` (
                                `id` int unsigned NOT NULL AUTO_INCREMENT,
                                `serial_number` varchar(100) NOT NULL,
                                `temperature` int NOT NULL,
                                `registered_at` datetime DEFAULT NULL,
                                PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=49 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

