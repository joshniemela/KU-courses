from scraper import get_diku_staff
    
if __name__=="__main__":
    staff = get_diku_staff()
    staff.to_csv("diku_staff.csv", index=False)
    print("Saved staff to diku_staff.csv")
