# Rust Actix Web + SeaORM DEMO项目

## 简介

本项目基于 Rust 编程语言和 Actix Web 框架构建，使用 SeaORM 进行数据库操作，旨在实现一个高性能的用户管理系统。主要功能包括用户注册、登录、信息查询等。

## 目录结构

## 功能

- **用户注册**：允许用户注册新账户。
- **用户登录**：允许用户登录账户。
- **信息查询**：允许用户查询自己的信息。 
- **中间件支持**：如日志记录、错误处理等。
- **数据库集成**：使用 SeaORM 进行数据库操作。

## 技术栈

- **Rust**: 一种系统编程语言，注重安全性和并发性。
- **Actix Web**: 一个高性能的 Web 框架。
- **SeaORM**: 一个现代化的 ORM 工具，用于数据库操作，支持 PostgreSQL。 
- **JWT**: 用于生成和验证 JSON Web Token。
- **SHA256**: 用于密码哈希。
- **UUID**: 用于生成唯一标识符。

## 安装与运行

### 前提条件

- Rust 和 Cargo 已安装
- PostgreSQL 数据库已安装并运行

### 安装步骤

1. 克隆仓库：
   ```sh
   git clone https://github.com/KJ-30/RUST_ACTIX.git
   cd RUST_ACTIX
2. 安装依赖：
   cargo build
3. 数据库迁移
   - 首先要在`.env`配置上数据库和密码
  sea-orm-cli migrate up
4. 运行项目
  cargo run
5.运行单元测试和集成测试：
  cargo test
