import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

response2 = WS.sendRequest(findTestObject('Comments/GET - Comments By ID'))

WS.verifyResponseStatusCode(response2, 200, FailureHandling.STOP_ON_FAILURE)

assert response2.getStatusCode() == 200

WS.verifyElementPropertyValue(response2, 'postId', 1, FailureHandling.STOP_ON_FAILURE)

WS.verifyElementPropertyValue(response2, 'id', 5, FailureHandling.STOP_ON_FAILURE)

WS.verifyElementPropertyValue(response2, 'name', 'vero eaque aliquid doloribus et culpa', FailureHandling.STOP_ON_FAILURE)

WS.verifyElementPropertyValue(response2, 'email', 'Hayden@althea.biz', FailureHandling.STOP_ON_FAILURE)

WS.verifyElementPropertyValue(response2, 'body', 'harum non quasi et ratione\ntempore iure ex voluptates in ratione\nharum architecto fugit inventore cupiditate\nvoluptates magni quo et',
	FailureHandling.STOP_ON_FAILURE)
